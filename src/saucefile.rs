use crate::context::Context;
use anyhow::Result;
use indexmap::IndexMap;

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use toml_edit::{value, Document, Value};

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
    pub fn new(documents: Vec<Document>) -> Self {
        Self { documents }
    }

    fn from_file_contents(path: &PathBuf, contents: String) -> Document {
        contents.parse::<Document>().unwrap_or_else(|e| {
            println!("Failed to parse {}", path.to_string_lossy());
            println!("{}", e);
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
            document["vars"][&key] = value(toml_value);
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

    pub fn vars(&mut self) -> Vec<(String, String)> {
        let mut map = IndexMap::new();
        for document in self.documents.iter() {
            for (key, item) in document.iter() {
                if let Some(value) = item.as_value() {
                    map.insert(key.to_string(), value.to_string());
                }
            }
        }
        map.into_iter().collect()
    }
}

impl Default for Saucefile {
    fn default() -> Self {
        Self {
            documents: Vec::new(),
        }
    }
}
