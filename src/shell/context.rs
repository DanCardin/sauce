use anyhow::Result;
use etcetera::home_dir;
use std::path::PathBuf;
use std::{env, ops::Deref};

use crate::{
    option::Options,
    output::Output,
    saucefile::Saucefile,
    settings::Settings,
    shell::{actions, Shell},
};

#[derive(Debug)]
pub struct Context<'a> {
    pub settings: Settings,
    pub options: Options<'a>,

    pub home: PathBuf,
    pub data_dir: PathBuf,

    pub path: PathBuf,
    pub sauce_path: PathBuf,
}

impl<'a> Context<'a> {
    pub fn from_path<P: Into<PathBuf>>(
        data_dir: PathBuf,
        path: P,
        settings: Settings,
        options: Options<'a>,
    ) -> Result<Self> {
        let path = path.into().canonicalize()?;

        let home = home_dir()?;

        let relative_path = path.strip_prefix(&home)?;
        let sauce_path = data_dir.join(relative_path).with_extension("toml");

        Ok(Self {
            settings,
            options,
            home,
            data_dir,
            path,
            sauce_path,
        })
    }

    pub fn from_current_dir(
        data_dir: PathBuf,
        settings: Settings,
        options: Options<'a>,
    ) -> Result<Self> {
        let current_dir = env::current_dir()?;
        Self::from_path(data_dir, current_dir, settings, options)
    }

    pub fn new(data_dir: PathBuf, settings: Settings, options: Options<'a>) -> Result<Self> {
        if let Some(path) = options.path {
            Self::from_path(data_dir, path, settings, options)
        } else {
            Self::from_current_dir(data_dir, settings, options)
        }
    }

    fn saucefile(&self) -> Saucefile {
        Saucefile::read(self)
    }

    pub fn init_shell(&self, shell_kind: &dyn Shell, output: &mut Output) {
        actions::init(self, shell_kind, output)
    }

    pub fn create_saucefile(&self, output: &mut Output) {
        let parent = self.sauce_path.parent().unwrap();
        if std::fs::create_dir_all(parent).is_err() {
            output.push_message(format!(
                "Couldn't create the thing {}",
                parent.to_string_lossy()
            ));
            return;
        }

        if self.sauce_path.is_file() {
            output.push_message(format!(
                "File already exists at {}",
                self.sauce_path.to_string_lossy()
            ));
        } else if std::fs::File::create(&self.sauce_path).is_err() {
            output.push_message("couldn't create the file");
        } else {
            output.push_message("Created".to_string());
        }
    }

    pub fn edit_saucefile(&self, shell_kind: &dyn Shell, output: &mut Output) {
        actions::edit(self, shell_kind, output);
    }

    pub fn show(&self, shell_kind: &dyn Shell, output: &mut Output) {
        actions::show(self, shell_kind, self.saucefile(), output);
    }

    pub fn clear(&self, shell_kind: &dyn Shell, output: &mut Output) {
        actions::clear(self, shell_kind, self.saucefile(), output);
    }

    pub fn execute(&self, shell_kind: &dyn Shell, output: &mut Output, autoload: bool) {
        actions::execute(self, shell_kind, self.saucefile(), output, autoload);
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

    pub fn set_var<T: AsRef<str>>(&self, values: &[T], output: &mut Output) {
        let mut saucefile = self.saucefile();
        for values in values.iter() {
            let parts: Vec<&str> = values.as_ref().splitn(2, '=').collect();
            let var = parts[0];

            let value = parts.get(1).map(Deref::deref).unwrap_or("");

            saucefile.set_var(var, value);
            output.push_message(format!("Set '{}' to {}", var, value));
        }
        if saucefile.write(self).is_err() {
            output.push_message("couldn't write the thing")
        }
    }

    pub fn set_alias<T: AsRef<str>>(&self, values: &[T], output: &mut Output) {
        let mut saucefile = self.saucefile();
        for values in values.iter() {
            let parts: Vec<&str> = values.as_ref().splitn(2, '=').collect();
            let var = parts[0];
            let value = if parts.len() > 1 { parts[1] } else { "" };
            saucefile.set_alias(var, value);
            output.push_message(format!("Set '{}' to {}", var, value));
        }
        if saucefile.write(self).is_err() {
            output.push_message("couldn't write the thing")
        }
    }

    pub fn set_function(&self, name: &str, body: &str, output: &mut Output) {
        let mut saucefile = self.saucefile();
        saucefile.set_function(name, body);
        output.push_message(format!("Set '{}' to {}", name, body));
        if saucefile.write(self).is_err() {
            output.push_message("couldn't write the thing")
        }
    }

    pub fn set_config<T: AsRef<str>>(&self, values: &[(T, T)], global: bool, output: &mut Output) {
        let saucefile = self.saucefile();
        let result = if global {
            self.settings.set_values(&values)
        } else {
            saucefile.settings().set_values(&values)
        };

        if result.is_err() {
            output.push_message(format!("Failed to set config values: {:?}", result));
        }
    }
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            options: Options::default(),
            home: PathBuf::new(),
            data_dir: PathBuf::new(),
            path: PathBuf::new(),
            sauce_path: PathBuf::new(),
        }
    }
}
