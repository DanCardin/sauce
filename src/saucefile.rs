use crate::settings::Settings;
use crate::Context;
use crate::{option::Options, toml::write_document};
use indexmap::IndexMap;
use itertools::iproduct;
use toml_edit::Table;

use crate::toml::get_document;
use std::path::PathBuf;
use std::str::FromStr;
use toml_edit::{value, Document, Item, Value};

#[derive(Debug)]
pub struct Saucefile {
    pub path: PathBuf,
    pub ancestors: Vec<Document>,
    pub document: Document,
}

impl Saucefile {
    pub fn read(context: &mut Context) -> Self {
        let mut base_sf = Self {
            path: context.sauce_path.clone(),
            ..Default::default()
        };

        let paths = context.cascade_paths();
        let mut paths = paths.peekable();
        while let Some(path) = paths.next() {
            if !path.is_file() {
                continue;
            }

            let document = get_document(&path, &mut context.output);

            if paths.peek().is_some() {
                base_sf.ancestors.push(document)
            } else {
                base_sf.document = document;
            }
        }
        base_sf
    }

    pub fn settings(&self) -> Settings {
        Settings::from_document(self.path.clone(), &self.document)
    }

    pub fn set_var(&mut self, name: &str, raw_value: &str) {
        let toml_value = Value::from_str(&raw_value).unwrap_or_else(|_| Value::from(raw_value));
        let env_section = self.document.as_table_mut().entry("environment");
        if env_section.is_none() {
            *env_section = Item::Table(Table::new());
        }
        self.document["environment"][&name] = value(toml_value);
    }

    pub fn set_alias(&mut self, name: &str, raw_value: &str) {
        let toml_value = Value::from_str(&raw_value).unwrap_or_else(|_| Value::from(raw_value));

        let alias_section = self.document.as_table_mut().entry("alias");
        if alias_section.is_none() {
            *alias_section = Item::Table(Table::new());
        }
        self.document["alias"][&name] = value(toml_value);
    }

    pub fn set_function(&mut self, name: &str, body: &str) {
        let toml_value = Value::from_str(&body).unwrap_or_else(|_| Value::from(body));

        let alias_section = self.document.as_table_mut().entry("function");
        if alias_section.is_none() {
            *alias_section = Item::Table(Table::new());
        }
        self.document["function"][&name] = value(toml_value);
    }

    pub fn write(&mut self, context: &mut Context) {
        write_document(&context.sauce_path, &self.document, &mut context.output);
    }

    fn section(&mut self, sections: &[&str], options: &Options) -> Vec<(&str, String)> {
        let tag = options.as_.unwrap_or("default");

        let documents = self.ancestors.iter().chain(vec![&self.document]);

        iproduct!(documents, sections)
            .map(|(document, section)| document[section].as_table())
            .filter_map(|x| x)
            .flat_map(|vars| vars.iter())
            .filter(|(key, _)| {
                options.glob_match(sections, key) && options.filter_match(sections, key)
            })
            .map(|(key, item)| {
                let var = match item {
                    Item::Value(value) => match value {
                        Value::InlineTable(table) => match table.get(&tag) {
                            Some(value) => unwrap_toml_value(value),
                            _ => "".to_string(),
                        },
                        _ => unwrap_toml_value(value),
                    },
                    Item::Table(table) => match &table[&tag] {
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

    pub fn vars(&mut self, options: &Options) -> Vec<(&str, String)> {
        self.section(&["env", "environment"], options)
    }

    pub fn aliases(&mut self, options: &Options) -> Vec<(&str, String)> {
        self.section(&["alias"], options)
    }

    pub fn functions(&mut self, options: &Options) -> Vec<(&str, String)> {
        self.section(&["function"], options)
    }
}

impl Default for Saucefile {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            document: Document::new(),
            ancestors: Vec::new(),
        }
    }
}

fn unwrap_toml_value(value: &Value) -> String {
    match value {
        Value::InlineTable(_) => value.as_inline_table().unwrap().to_string(),
        Value::Array(_) => value.as_array().unwrap().to_string(),
        Value::String(_) => value.as_str().unwrap().to_string(),
        Value::Integer(_) => value.as_integer().unwrap().to_string(),
        Value::Boolean(_) => value.as_bool().unwrap().to_string(),
        Value::Float(_) => value.as_float().unwrap().to_string(),
        Value::DateTime(_) => value.as_date_time().unwrap().to_string(),
    }
}

#[cfg(test)]
mod tests {
    mod section {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_includes_values_in_section() {
            let options = Options::default();
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = "baz"
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["foo"], &options);
            assert_eq!(result, vec![("bar", "baz".to_string())]);
        }

        #[test]
        fn it_includes_when_one_section_option_matches() {
            let options = Options::default();
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = "baz"
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["non-matching", "foo"], &options);
            assert_eq!(result, vec![("bar", "baz".to_string())]);
        }

        #[test]
        fn it_excludes_when_no_section_matches() {
            let options = Options::default();
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = "baz"
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["non-matching"], &options);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn it_chooses_the_default_tag() {
            let options = Options::default();
            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = {default = 1}
            bees = 2
            boops = {notdefault = 3}
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["foo"], &options);
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
            let mut options = Options::default();
            options.as_ = Some("wow");

            let mut sauce = Saucefile::default();

            let toml = r#"
            [foo]
            bar = {wow = 1}
            bees = 2
            boops = {notwow = 3}
            "#;
            sauce.document = toml.parse::<Document>().expect("invalid doc");

            let result = sauce.section(&["foo"], &options);
            assert_eq!(
                result,
                vec![
                    ("bar", "1".to_string()),
                    ("bees", "2".to_string()),
                    ("boops", "".to_string()),
                ]
            );
        }
    }

    mod vars {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_yields_empty_when_empty() {
            let options = Options::default();
            let mut sauce = Saucefile::default();
            let result = sauce.vars(&options);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn it_roundtrips_value() {
            let options = Options::default();
            let mut sauce = Saucefile::default();

            sauce.set_var("meow", "5");
            let result = sauce.vars(&options);

            assert_eq!(result, vec![("meow", "5".to_string())]);
        }
    }

    mod aliases {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_yields_empty_when_empty() {
            let options = Options::default();
            let mut sauce = Saucefile::default();
            let result = sauce.aliases(&options);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn it_roundtrips_value() {
            let options = Options::default();
            let mut sauce = Saucefile::default();

            sauce.set_alias("meow", "5");
            let result = sauce.aliases(&options);

            assert_eq!(result, vec![("meow", "5".to_string())]);
        }
    }

    mod functions {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_yields_empty_when_empty() {
            let options = Options::default();
            let mut sauce = Saucefile::default();
            let result = sauce.functions(&options);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn it_roundtrips_value() {
            let options = Options::default();
            let mut sauce = Saucefile::default();

            sauce.set_function("meow", "5");
            let result = sauce.functions(&options);

            assert_eq!(result, vec![("meow", "5".to_string())]);
        }
    }
}
