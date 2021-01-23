use anyhow::Result;
use std::{fmt::Display, path::PathBuf};

use crate::toml::get_document;
use etcetera::app_strategy::AppStrategy;
use etcetera::app_strategy::AppStrategyArgs;
use etcetera::app_strategy::Xdg;
use toml_edit::{Document, Item, Value};

#[derive(Debug)]
pub struct Settings {
    pub data_dir: Option<PathBuf>,
    pub autoload_hook: Option<bool>,
    pub autoload: Option<bool>,
}

#[derive(Debug)]
pub struct RealizedSettings {
    pub data_dir: Option<PathBuf>,
    pub autoload_hook: bool,
    pub autoload: bool,
}

impl Default for RealizedSettings {
    fn default() -> Self {
        Self {
            data_dir: None,
            autoload_hook: false,
            autoload: false,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let strat_args = AppStrategyArgs {
            top_level_domain: "com".to_string(),
            author: "dancardin".to_string(),
            app_name: "sauce".to_string(),
        };
        let strategy = Xdg::new(strat_args)?;
        let data_dir = strategy.data_dir();

        let config_dir = strategy.config_dir().with_extension("toml");
        let document = get_document(&config_dir);
        Ok(Self::from_document(&document, Some(data_dir)))
    }

    pub fn from_document(document: &Document, data_dir: Option<PathBuf>) -> Self {
        let general = &document["settings"];

        let autoload_hook = Setting::new(general, "autoload-hook").as_bool();
        let autoload = Setting::new(general, "autoload").as_bool();

        Self {
            data_dir,
            autoload_hook,
            autoload,
        }
    }

    pub fn resolve_precedence(&self, fallback: &Self) -> RealizedSettings {
        let mut default = RealizedSettings::default();

        let settings_precedence = vec![fallback, self];

        for settings in settings_precedence {
            if let Some(v) = settings.autoload_hook {
                default.autoload_hook = v;
            }
            if let Some(v) = settings.autoload {
                default.autoload = v;
            }
        }
        default
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

    fn notify_invalid<T: Display>(&self, kind: &str, value: Option<T>) -> Option<T> {
        if value.is_none() {
            eprintln!(
                "Settings Error: Failed to interpret '{}' value as {}",
                self.name, kind,
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
                    "Settings Error: Expected '{}' to be a value, not a table",
                    self.name
                );
                None
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            data_dir: None,
            autoload_hook: None,
            autoload: None,
        }
    }
}

#[cfg(test)]
mod tests {
    mod settings_from_document {
        use super::super::*;
        use pretty_assertions::assert_eq;
        use toml_edit::Document;

        #[test]
        fn it_loads_from_empty_document() {
            let toml = r#""#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let settings = Settings::from_document(&doc, None);
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
            let settings = Settings::from_document(&doc, None);
            assert_eq!(settings.autoload, Some(true));
            assert_eq!(settings.autoload_hook, Some(true));
        }
    }

    mod settings_resolve_precedence {
        use super::super::*;
        use pretty_assertions::assert_eq;
        use toml_edit::Document;

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
            let global = Settings::from_document(&doc, None);

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
            let saucefile_settings = Settings::from_document(&doc, None);

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
            let global = Settings::from_document(&doc, None);

            let toml = r#"
                [settings]
                autoload = true
                autoload-hook = false
                "#;
            let doc = toml.parse::<Document>().expect("invalid doc");
            let saucefile_settings = Settings::from_document(&doc, None);

            let settings = saucefile_settings.resolve_precedence(&global);
            assert_eq!(settings.autoload, true);
            assert_eq!(settings.autoload_hook, false);
        }
    }
}
