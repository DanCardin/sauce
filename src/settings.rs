use std::fmt::Display;
use std::path::{Path, PathBuf};

use toml_edit::{Document, Item, Table, Value};

use crate::colors::{RED, YELLOW};
use crate::output::{ErrorCode, Output};
use crate::toml::get_document;

#[derive(Debug)]
pub struct RealizedSettings {
    pub autoload_hook: bool,
    pub autoload: bool,
    pub clear_ignore: Vec<String>,
}

impl Default for RealizedSettings {
    fn default() -> Self {
        Self {
            autoload_hook: false,
            autoload: false,
            clear_ignore: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    pub file: PathBuf,
    pub autoload_hook: Option<bool>,
    pub autoload: Option<bool>,
    pub clear_ignore: Option<Vec<String>>,
}

impl Settings {
    pub fn load(config_dir: &Path, output: &mut Output) -> Self {
        let file = config_dir.with_extension("toml");
        let document = get_document(&file, output);
        Self::from_document(file, &document)
    }

    pub fn from_document(file: PathBuf, document: &Document) -> Self {
        let general = &document["settings"];

        let autoload_hook = Setting::new(general, "autoload-hook").as_bool();
        let autoload = Setting::new(general, "autoload").as_bool();
        let clear_ignore = Setting::new(general, "clear-ignore").as_vec_of_string();

        Self {
            file,
            autoload_hook,
            autoload,
            clear_ignore,
        }
    }

    pub fn resolve_precedence<'a>(&'a self, fallback: &'a Self) -> RealizedSettings {
        let mut default = RealizedSettings::default();

        let settings_precedence = vec![fallback, self];

        for settings in settings_precedence {
            if let Some(v) = settings.autoload_hook {
                default.autoload_hook = v;
            }
            if let Some(v) = settings.autoload {
                default.autoload = v;
            }
            if let Some(v) = &settings.clear_ignore {
                default.clear_ignore = v.to_vec();
            }
        }
        default
    }

    pub fn set_values<T: AsRef<str>>(&self, pairs: &[(T, T)], output: &mut Output) {
        let mut document = get_document(&self.file, output);
        let settings_section = document.as_table_mut().entry("settings");
        if settings_section.is_none() {
            *settings_section = Item::Table(Table::new());
        }

        let values = pairs
            .iter()
            .filter_map(|(setting, value)| match setting.as_ref() {
                "autoload" | "autoload-hook" | "clear-ignore" => {
                    if let Ok(parsed_value) = value.as_ref().parse::<Value>() {
                        Some((setting.as_ref(), toml_edit::value(parsed_value)))
                    } else {
                        output.notify_error(
                            ErrorCode::ParseError,
                            &[
                                RED.bold().paint("Could not parse config value"),
                                YELLOW.bold().paint(value.as_ref()),
                            ],
                        );
                        None
                    }
                }
                unknown_setting => {
                    output.notify_error(
                        ErrorCode::ParseError,
                        &[
                            RED.bold().paint("Unrecognized config name"),
                            YELLOW.bold().paint(unknown_setting),
                        ],
                    );
                    None
                }
            })
            .collect::<Vec<_>>();

        if values.is_empty() {
            return;
        }

        output.write_toml(&self.file, &mut document, "settings", values);
    }
}

struct Setting<'a> {
    name: &'a str,
    item: &'a Item,
}

impl<'a> Setting<'a> {
    pub fn new(item: &'a Item, name: &'a str) -> Self {
        Self {
            name,
            item: &item[name],
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Some(value) = self.get_value() {
            self.notify_invalid("bool", value.as_bool())
        } else {
            None
        }
    }

    pub fn as_vec_of_string(&self) -> Option<Vec<String>> {
        if let Some(value) = self.get_value() {
            self.notify_invalid("list of string", value.as_array())
                .map(|t| {
                    t.iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect()
                })
        } else {
            None
        }
    }

    fn notify_invalid<T: Display>(&self, kind: &str, value: Option<T>) -> Option<T> {
        if value.is_none() {
            eprintln!(
                "{}",
                format!(
                    "{} {} {} {}",
                    RED.bold()
                        .paint("Settings Error: Failed to interpret value"),
                    YELLOW.paint(self.name),
                    RED.bold().paint("value as"),
                    YELLOW.paint(kind),
                )
            );
        }
        value
    }

    fn get_value(&self) -> Option<&Value> {
        match &self.item {
            Item::None => None,
            Item::Value(value) => Some(value),
            _ => {
                eprintln!(
                    "{}",
                    format!(
                        "{} {} {}",
                        RED.bold().paint("Settings Error: Expected"),
                        YELLOW.paint(self.name),
                        RED.bold().paint("to be a value, not a table"),
                    )
                );
                None
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            autoload_hook: None,
            autoload: None,
            clear_ignore: None,
        }
    }
}

#[cfg(test)]
mod tests {
    mod settings_from_document {
        use pretty_assertions::assert_eq;
        use toml_edit::Document;

        use super::super::*;

        #[test]
        fn it_loads_from_empty_document() {
            let toml = r#""#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let settings = Settings::from_document(PathBuf::new(), &doc);
            assert_eq!(settings.autoload, None);
            assert_eq!(settings.autoload_hook, None);
        }

        #[test]
        fn it_loads_from_document() {
            let toml = r#"
                [settings]
                autoload = true
                autoload-hook = true
            "#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let settings = Settings::from_document(PathBuf::new(), &doc);
            assert_eq!(settings.autoload, Some(true));
            assert_eq!(settings.autoload_hook, Some(true));
        }
    }

    mod settings_resolve_precedence {
        use pretty_assertions::assert_eq;
        use toml_edit::Document;

        use super::super::*;

        #[test]
        fn it_defaults_to_default() {
            let global = Settings::default();
            let saucefile_settings = Settings::default();

            let settings = saucefile_settings.resolve_precedence(&global);
            assert_eq!(settings.autoload, false);
            assert_eq!(settings.autoload_hook, false);
        }

        #[test]
        fn it_takes_the_global_value_when_theres_no_local_one() {
            let toml = r#"
                [settings]
                autoload = true
                autoload-hook = true
                "#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let global = Settings::from_document(PathBuf::new(), &doc);

            let saucefile_settings = Settings::default();

            let settings = saucefile_settings.resolve_precedence(&global);
            assert_eq!(settings.autoload, true);
            assert_eq!(settings.autoload_hook, true);
        }

        #[test]
        fn it_takes_the_local_value_when_theres_no_global_one() {
            let global = Settings::default();

            let toml = r#"
                [settings]
                autoload = true
                autoload-hook = true
                "#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let saucefile_settings = Settings::from_document(PathBuf::new(), &doc);

            let settings = saucefile_settings.resolve_precedence(&global);
            assert_eq!(settings.autoload, true);
            assert_eq!(settings.autoload_hook, true);
        }

        #[test]
        fn it_local_beats_global() {
            let toml = r#"
                [settings]
                autoload = false
                autoload-hook = true
                "#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let global = Settings::from_document(PathBuf::new(), &doc);

            let toml = r#"
                [settings]
                autoload = true
                autoload-hook = false
                "#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let saucefile_settings = Settings::from_document(PathBuf::new(), &doc);

            let settings = saucefile_settings.resolve_precedence(&global);
            assert_eq!(settings.autoload, true);
            assert_eq!(settings.autoload_hook, false);
        }
    }
}
