use anyhow::Result;
use etcetera::home_dir;
use std::path::PathBuf;
use std::{env, ops::Deref};

use crate::{
    colors::{BLUE, RED, YELLOW},
    option::Options,
    output::{ErrorCode, Output},
    saucefile::Saucefile,
    settings::Settings,
    shell::{actions, Shell},
};

#[derive(Debug)]
pub struct Context<'a> {
    pub settings: Settings,
    pub options: Options<'a>,
    pub output: Output,

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
        output: Output,
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
            output,
        })
    }

    pub fn from_current_dir(
        data_dir: PathBuf,
        settings: Settings,
        options: Options<'a>,
        output: Output,
    ) -> Result<Self> {
        let current_dir = env::current_dir()?;
        Self::from_path(data_dir, current_dir, settings, options, output)
    }

    pub fn new(
        data_dir: PathBuf,
        settings: Settings,
        options: Options<'a>,
        output: Output,
    ) -> Result<Self> {
        if let Some(path) = options.path {
            Self::from_path(data_dir, path, settings, options, output)
        } else {
            Self::from_current_dir(data_dir, settings, options, output)
        }
    }

    fn saucefile(&mut self) -> Saucefile {
        Saucefile::read(self)
    }

    pub fn init_shell(&mut self, shell_kind: &dyn Shell) {
        actions::init(self, shell_kind)
    }

    pub fn execute_shell(&mut self, shell_kind: &dyn Shell, command: Option<&str>) {
        actions::execute_shell(self, shell_kind, command)
    }

    pub fn create_saucefile(&mut self) {
        let parent = self.sauce_path.parent().unwrap();
        if std::fs::create_dir_all(parent).is_err() {
            self.output.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.paint("Couldn't create "),
                    YELLOW.paint(parent.to_string_lossy()),
                ],
            );
            return;
        }

        if self.sauce_path.is_file() {
            self.output.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.bold().paint("File already exists at "),
                    YELLOW.paint(self.sauce_path.to_string_lossy()),
                ],
            );
        } else if std::fs::File::create(&self.sauce_path).is_err() {
            self.output.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.bold().paint("Couldn't create"),
                    YELLOW.paint(self.sauce_path.to_string_lossy()),
                ],
            );
        } else {
            self.output.notify(&[
                BLUE.bold().paint("Created"),
                YELLOW.paint(self.sauce_path.to_string_lossy()),
            ]);
        }
    }

    pub fn edit_saucefile(&mut self, shell_kind: &dyn Shell) {
        actions::edit(self, shell_kind);
    }

    pub fn show(&mut self, shell_kind: &dyn Shell) {
        let saucefile = self.saucefile();
        actions::show(self, shell_kind, saucefile);
    }

    pub fn clear(&mut self, shell_kind: &dyn Shell) {
        let saucefile = self.saucefile();
        actions::clear(self, shell_kind, saucefile);
    }

    pub fn execute(&mut self, shell_kind: &dyn Shell, autoload: bool) {
        let saucefile = self.saucefile();
        actions::execute(self, shell_kind, saucefile, autoload);
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

    pub fn set_var<T: AsRef<str>>(&mut self, values: &[T]) {
        let mut saucefile = self.saucefile();
        for values in values.iter() {
            let parts: Vec<&str> = values.as_ref().splitn(2, '=').collect();
            let var = parts[0];

            let value = parts.get(1).map(Deref::deref).unwrap_or("");

            saucefile.set_var(var, value);
            self.output.notify(&[
                BLUE.paint("Set "),
                YELLOW.paint(var),
                BLUE.paint(" = "),
                YELLOW.paint(value),
            ]);
        }
        saucefile.write(self);
    }

    pub fn set_alias<T: AsRef<str>>(&mut self, values: &[T]) {
        let mut saucefile = self.saucefile();
        for values in values.iter() {
            let parts: Vec<&str> = values.as_ref().splitn(2, '=').collect();
            let var = parts[0];
            let value = if parts.len() > 1 { parts[1] } else { "" };
            saucefile.set_alias(var, value);
            self.output.notify(&[
                BLUE.paint("Set "),
                YELLOW.paint(var),
                BLUE.paint(" = "),
                YELLOW.paint(value),
            ]);
        }
        saucefile.write(self);
    }

    pub fn set_function(&mut self, name: &str, body: &str) {
        let mut saucefile = self.saucefile();
        saucefile.set_function(name, body);
        self.output.notify(&[
            BLUE.paint("Set "),
            YELLOW.paint(name),
            BLUE.paint(" = "),
            YELLOW.paint(body),
        ]);
        saucefile.write(self);
    }

    pub fn set_config<T: AsRef<str>>(&mut self, values: &[(T, T)], global: bool) {
        let saucefile = self.saucefile();
        if global {
            self.settings.set_values(&values, &mut self.output);
        } else {
            saucefile.settings().set_values(&values, &mut self.output);
        };
    }

    pub fn flush(&mut self) -> Result<()> {
        self.output.flush()
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
            output: Output::default(),
        }
    }
}
