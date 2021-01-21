use anyhow::Result;
use etcetera::home_dir;
use std::env;
use std::path::PathBuf;

use crate::option::Options;

#[derive(Debug)]
pub struct Context {
    home: PathBuf,
    data_dir: PathBuf,
    path: PathBuf,
    pub sauce_path: PathBuf,
}

impl Context {
    pub fn from_path<P: Into<PathBuf>>(path: P, options: &Options) -> Result<Self> {
        let path = path.into().canonicalize()?;

        let home = home_dir()?;
        let data_dir = options.settings.data_dir.clone();

        let relative_path = path.strip_prefix(&home)?;
        let sauce_path = data_dir.join(relative_path).with_extension("toml");

        Ok(Self {
            home,
            data_dir,
            path,
            sauce_path,
        })
    }

    pub fn from_current_dir(options: &Options) -> Result<Self> {
        let current_dir = env::current_dir()?;
        Self::from_path(current_dir, options)
    }

    pub fn new(options: &Options) -> Result<Self> {
        if let Some(path) = options.path {
            Self::from_path(path, options)
        } else {
            Self::from_current_dir(options)
        }
    }

    pub fn cascade_paths(&self) -> Vec<PathBuf> {
        if self.sauce_path == self.data_dir.with_extension("toml") {
            return vec![self.sauce_path.clone()];
        }

        self.sauce_path
            .ancestors()
            .filter(|p| p.strip_prefix(&self.data_dir).is_ok())
            .map(|p| p.with_extension("toml"))
            .collect::<Vec<PathBuf>>()
            .into_iter()
            .rev()
            .collect()
    }
}
