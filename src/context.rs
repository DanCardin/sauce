use anyhow::Result;
use etcetera::app_strategy::AppStrategy;
use etcetera::app_strategy::AppStrategyArgs;
use etcetera::app_strategy::Xdg;
use etcetera::home_dir;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    home: PathBuf,
    data_dir: PathBuf,
    path: PathBuf,
    pub sauce_path: PathBuf,
}

impl Context {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into().canonicalize()?;

        let home = home_dir()?;

        let strategy = Xdg::new(AppStrategyArgs {
            top_level_domain: "com".to_string(),
            author: "dancardin".to_string(),
            app_name: "sauce".to_string(),
        })?;
        let data_dir = strategy.data_dir();

        let relative_path = path.strip_prefix(&home)?;
        let sauce_path = data_dir.join(relative_path).with_extension("toml");

        Ok(Self {
            home,
            data_dir,
            path,
            sauce_path,
        })
    }

    pub fn from_current_dir() -> Result<Self> {
        let current_dir = env::current_dir()?;
        Self::from_path(current_dir)
    }

    pub fn cascade_paths(&self) -> Vec<PathBuf> {
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
