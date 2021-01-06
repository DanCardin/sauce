use anyhow::Result;
use etcetera::app_strategy::AppStrategy;
use etcetera::app_strategy::AppStrategyArgs;
use etcetera::app_strategy::Xdg;
use etcetera::home_dir;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    pub home: PathBuf,
    pub data_dir: PathBuf,
    pub current_dir: PathBuf,
    pub sauce_path: PathBuf,
}

impl Context {
    pub fn new() -> Result<Self> {
        let home = home_dir()?;

        let strategy = Xdg::new(AppStrategyArgs {
            top_level_domain: "com".to_string(),
            author: "dancardin".to_string(),
            app_name: "sauce".to_string(),
        })?;
        let data_dir = strategy.data_dir();

        let current_dir = env::current_dir()?;

        let relative_path = current_dir.strip_prefix(&home)?;
        let sauce_path = data_dir.join(relative_path).with_extension("toml");

        Ok(Self {
            home,
            data_dir,
            current_dir,
            sauce_path,
        })
    }

    pub fn cascade_paths(self: &Self) -> Vec<PathBuf> {
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
