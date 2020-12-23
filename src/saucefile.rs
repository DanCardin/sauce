use crate::context::Context;
use anyhow::Result;

use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Saucefile {
    #[serde(default)]
    pub vars: HashMap<String, String>,
}

fn read_file(path: &Path) -> String {
    if let Ok(file) = std::fs::File::open(path) {
        let mut reader = BufReader::new(file);

        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap_or_else(|e| {
            println!("{}", e);
            0
        });
        contents
    } else {
        String::new()
    }
}

impl Saucefile {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    fn from_file_contents(path: &PathBuf, contents: String) -> Self {
        toml::from_str(&contents).unwrap_or_else(|_| {
            println!("Failed to parse {}", path.to_string_lossy());
            Saucefile::new()
        })
    }

    pub fn read(context: &Context) -> Saucefile {
        let mut base_sf: Saucefile = Self::new();

        for path in context.cascade_paths() {
            if !path.is_file() {
                continue;
            }

            let sf = Self::from_file_contents(&path, read_file(&path));
            base_sf = Self::compose(base_sf, sf);
        }
        base_sf
    }

    pub fn compose(s1: Self, s2: Self) -> Self {
        let mut new = Self::new();

        let mut new_vars: HashMap<String, String> = HashMap::new();
        for s in ArrayVec::from([s1, s2]) {
            let Self { vars } = s;
            new_vars.extend(vars.into_iter());
        }

        new.vars = new_vars;
        new
    }

    pub fn add_var(&mut self, key: String, value: String) {
        self.vars.insert(key, value);
    }

    pub fn write(&mut self, context: &Context) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(context.sauce_path.clone())?;
        let mut buffer = BufWriter::new(file);

        buffer.write_all(toml::to_string(&self)?.as_ref())?;
        buffer.flush()?;
        Ok(())
    }
}
