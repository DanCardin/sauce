use crate::{filter::FilterOptions, output::Output};
use crate::{settings::Settings, toml::unwrap_toml_value};
use indexmap::IndexMap;
use itertools::iproduct;

use crate::toml::get_document;
use std::path::PathBuf;
use toml_edit::{Document, Item, Value};

#[derive(Debug)]
pub struct Saucefile {
    pub path: Option<PathBuf>,
    pub ancestors: Vec<(PathBuf, Document)>,
    pub document: Document,
}

impl Saucefile {
    pub fn read<T>(output: &mut Output, ancestors: T) -> Self
    where
        T: IntoIterator<Item = PathBuf>,
    {
        let mut paths = ancestors.into_iter().peekable();

        let mut base_sf = Self::default();

        while let Some(path) = paths.next() {
            if !path.is_file() {
                continue;
            }

            let document = get_document(&path, output);

            if paths.peek().is_some() {
                base_sf.ancestors.push((path, document));
            } else {
                base_sf.document = document;
                base_sf.path = Some(path.to_path_buf());
            }
        }
        base_sf
    }

    pub fn settings(&self) -> Settings {
        if let Some(path) = &self.path {
            Settings::from_document(path.clone(), &self.document)
        } else {
            Settings::default()
        }
    }

    fn ancestors(&self) -> impl Iterator<Item = (&PathBuf, &Document)> {
        let ancestors = self.ancestors.iter().map(|(p, d)| (p, d));

        let mut tail = Vec::new();
        if let Some(path) = &self.path {
            tail.push((path, &self.document));
        }
        ancestors.chain(tail)
    }

    pub fn paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.ancestors().map(|(p, _)| p)
    }

    fn documents(&self) -> impl Iterator<Item = &Document> {
        self.ancestors().map(|(_, d)| d)
    }

    fn section(&self, sections: &[&str], filter_options: &FilterOptions) -> Vec<(&str, String)> {
        let tag = filter_options.as_.unwrap_or("default");

        iproduct!(self.documents(), sections)
            .filter_map(|(document, section)| document[section].as_table())
            .flat_map(|vars| vars.iter())
            .filter(|(key, _)| {
                filter_options.glob_match(sections, key)
                    && filter_options.filter_match(sections, key)
                    && filter_options.filter_exclude(sections, key)
            })
            .map(|(key, item)| {
                let var = match item {
                    Item::Value(value) => match value {
                        Value::InlineTable(table) => match table.get(tag) {
                            Some(value) => unwrap_toml_value(value),
                            _ => "".to_string(),
                        },
                        _ => unwrap_toml_value(value),
                    },
                    Item::Table(table) => match &table[tag] {
                        Item::Value(value) => unwrap_toml_value(value),
                        _ => "".to_string(),
                    },
                    _ => "".to_string(),
                };
                (key, var)
            })
            .collect::<IndexMap<&str, String>>()
            .into_iter()
            .collect()
    }

    pub fn vars(&self, filter_options: &FilterOptions) -> Vec<(&str, String)> {
        self.section(&["env", "environment"], filter_options)
    }

    pub fn aliases(&self, filter_options: &FilterOptions) -> Vec<(&str, String)> {
        self.section(&["alias"], filter_options)
    }

    pub fn functions(&self, filter_options: &FilterOptions) -> Vec<(&str, String)> {
        self.section(&["function"], filter_options)
    }

    pub fn files(&self, filter_options: &FilterOptions) -> Vec<(&str, String)> {
        self.section(&["file"], filter_options)
    }
}

impl Default for Saucefile {
    fn default() -> Self {
        Self {
            path: Some(PathBuf::new()),
            document: Document::new(),
            ancestors: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod section {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_includes_values_in_section() {
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = "baz"
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["foo"], &FilterOptions::default());
            assert_eq!(result, vec![("bar", "baz".to_string())]);
        }

        #[test]
        fn it_includes_when_one_section_option_matches() {
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = "baz"
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["non-matching", "foo"], &FilterOptions::default());
            assert_eq!(result, vec![("bar", "baz".to_string())]);
        }

        #[test]
        fn it_excludes_when_no_section_matches() {
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = "baz"
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["non-matching"], &FilterOptions::default());
            assert_eq!(result, &[]);
        }

        #[test]
        fn it_chooses_the_default_tag() {
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = {default = 1}
            bees = 2
            boops = {notdefault = 3}
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["foo"], &FilterOptions::default());
            assert_eq!(
                result,
                vec![
                    ("bar", "1".to_string()),
                    ("bees", "2".to_string()),
                    ("boops", "".to_string()),
                ]
            );
        }

        #[test]
        fn it_chooses_the_correct_tag() {
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = {wow = 1}
            bees = 2
            boops = {notwow = 3}
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(
                &["foo"],
                &FilterOptions {
                    as_: Some("wow"),
                    ..Default::default()
                },
            );
            assert_eq!(
                result,
                vec![
                    ("bar", "1".to_string()),
                    ("bees", "2".to_string()),
                    ("boops", "".to_string()),
                ]
            );
        }

        #[test]
        fn it_excludes_filter_exclusions() {
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = 1
            bees = 2
            meow = 3
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(
                &["foo"],
                &FilterOptions {
                    filter_exclusions: &[(None, "bar")],
                    ..Default::default()
                },
            );
            assert_eq!(result, vec![("bees", "2".into()), ("meow", "3".into())]);
        }

        #[test]
        fn it_includes_non_matching_filter_exclusions() {
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = 1
            bees = 2
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(
                &["foo"],
                &FilterOptions {
                    filter_exclusions: &[(Some("bar"), "bar")],
                    ..Default::default()
                },
            );
            assert_eq!(
                result,
                vec![("bar", "1".to_string()), ("bees", "2".to_string())]
            );
        }
    }

    mod vars {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_yields_empty_when_empty() {
            let sauce = Saucefile::default();
            let result = sauce.vars(&FilterOptions::default());
            assert_eq!(result, &[]);
        }
    }

    mod aliases {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_yields_empty_when_empty() {
            let sauce = Saucefile::default();
            let result = sauce.aliases(&FilterOptions::default());
            assert_eq!(result, &[]);
        }
    }

    mod functions {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_yields_empty_when_empty() {
            let sauce = Saucefile::default();
            let result = sauce.functions(&FilterOptions::default());
            assert_eq!(result, &[]);
        }
    }
}
