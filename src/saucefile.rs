use crate::context::Context;
use anyhow::Result;
use indexmap::IndexMap;
use toml_edit::Table;

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use toml_edit::{value, Document, Item, Value};

#[derive(Debug)]
pub struct Saucefile {
    pub documents: Vec<Document>,
}

fn read_file(path: &Path) -> String {
    if let Ok(file) = std::fs::File::open(path) {
        let mut reader = BufReader::new(file);

        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap_or(0);
        contents
    } else {
        String::new()
    }
}

impl Saucefile {
    #[allow(unused)]
    pub fn new(documents: Vec<Document>) -> Self {
        Self { documents }
    }

    fn from_file_contents(path: &PathBuf, contents: String) -> Document {
        contents.parse::<Document>().unwrap_or_else(|e| {
            eprintln!("Failed to parse {}", path.to_string_lossy());
            eprintln!("{}", e);
            Document::new()
        })
    }

    pub fn read(context: &Context) -> Saucefile {
        let mut base_sf: Saucefile = Self::default();

        for path in context.cascade_paths() {
            if !path.is_file() {
                continue;
            }

            let document = Self::from_file_contents(&path, read_file(&path));
            base_sf.documents.push(document)
        }
        base_sf
    }

    pub fn set_var(&mut self, key: String, raw_value: String) {
        if let Some(document) = self.documents.last_mut() {
            let toml_value = Value::from_str(&raw_value).unwrap_or_else(|_| Value::from(raw_value));
            let env_section = document.as_table_mut().entry("environment");
            if env_section.is_none() {
                *env_section = Item::Table(Table::new());
            }
            document["environment"][&key] = value(toml_value);
        }
    }

    pub fn set_alias(&mut self, key: String, raw_value: String) {
        if let Some(document) = self.documents.last_mut() {
            let toml_value = Value::from_str(&raw_value).unwrap_or_else(|_| Value::from(raw_value));

            let alias_section = document.as_table_mut().entry("alias");
            if alias_section.is_none() {
                *alias_section = Item::Table(Table::new());
            }
            document["alias"][&key] = value(toml_value);
        }
    }

    pub fn write(&mut self, context: &Context) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(context.sauce_path.clone())?;
        let mut buffer = BufWriter::new(file);

        if let Some(document) = self.documents.last() {
            buffer.write_all(document.to_string().as_ref())?;
            buffer.flush()?;
        }

        Ok(())
    }

    fn section(&mut self, section: &str, tag: Option<&str>) -> Vec<(String, String)> {
        let tag = tag.unwrap_or("default");

        let mut map = IndexMap::new();

        for document in self.documents.iter() {
            if let Some(vars) = document[section].as_table() {
                for (key, item) in vars.iter() {
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
                    map.insert(key.to_string(), var);
                }
            }
        }
        map.into_iter().collect()
    }

    pub fn vars(&mut self, tag: Option<&str>) -> Vec<(String, String)> {
        self.section("environment", tag)
    }

    pub fn aliases(&mut self, tag: Option<&str>) -> Vec<(String, String)> {
        self.section("alias", tag)
    }
}

impl Default for Saucefile {
    fn default() -> Self {
        Self {
            documents: Vec::new(),
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
