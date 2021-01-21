use anyhow::Result;
use std::{fmt::Display, path::PathBuf};

use crate::toml::get_document;
use etcetera::app_strategy::AppStrategy;
use etcetera::app_strategy::AppStrategyArgs;
use etcetera::app_strategy::Xdg;
use toml_edit::{Item, Value};

#[derive(Debug)]
pub struct Settings {
    pub data_dir: PathBuf,
    pub autoload_hook: bool,
    pub autoload: bool,
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
        let config = get_document(&config_dir);

        let default = Self::default();
        let general = &config["general"];

        let autoload_hook = Setting::new(general, "autoload-hook").as_bool(default.autoload_hook);
        let autoload = Setting::new(general, "autoload").as_bool(default.autoload);

        Ok(Self {
            data_dir,
            autoload_hook,
            autoload,
        })
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

    pub fn as_bool(&self, default: bool) -> bool {
        if let Some(value) = self.get_value() {
            self.default_to("bool", value.as_bool(), default)
        } else {
            default
        }
    }

    fn default_to<T: Display>(&self, kind: &str, value: Option<T>, default: T) -> T {
        value.unwrap_or_else(|| {
            eprintln!(
                "Settings Error: Failed to interpret '{}' value as {}, defaulting to {}",
                self.name, kind, default
            );
            default
        })
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
            data_dir: "~/.local/share/sauce".into(),
            autoload_hook: false,
            autoload: false,
        }
    }
}
