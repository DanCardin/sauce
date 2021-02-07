use anyhow::Result;
use etcetera::home_dir;
use std::path::PathBuf;
use std::{env, path::Path};

use crate::{
    colors::{BLUE, RED, YELLOW},
    option::Options,
    output::{ErrorCode, Output},
    saucefile::Saucefile,
    settings::Settings,
    shell::{actions, Shell},
    target::Target,
};

#[derive(Debug)]
pub struct Context<'a> {
    pub settings: Settings,
    pub options: Options<'a>,
    pub output: Output,

    pub data_dir: PathBuf,

    pub path: PathBuf,
    pub sauce_path: PathBuf,
}

impl<'a> Context<'a> {
    pub fn new(
        data_dir: PathBuf,
        settings: Settings,
        options: Options<'a>,
        output: Output,
    ) -> Result<Self> {
        let (path, sauce_path, data_dir) = match options.file {
            // The default case, where no `file` is supplied. We perform normal
            // path lookup and saucefile cascading behavior.
            None => {
                let path = if let Some(path) = options.path {
                    Path::new(path).to_path_buf()
                } else {
                    env::current_dir()?
                };

                let path = path.canonicalize()?;

                let home = home_dir()?;

                let relative_path = path.strip_prefix(&home)?;
                let sauce_path = data_dir.join(relative_path).with_extension("toml");
                (path, sauce_path, data_dir)
            }
            // The default case, where no `file` is supplied. We perform normal
            // path lookup and saucefile cascading behavior.
            Some(file) => {
                let file = Path::new(file).to_path_buf().canonicalize()?;

                (file.clone(), file, "".into())
            }
        };
        Ok(Self {
            data_dir,
            settings,
            options,
            output,
            path,
            sauce_path,
        })
    }

    fn saucefile(&mut self) -> Saucefile {
        Saucefile::read(self)
    }

    pub fn init_shell(&mut self, shell_kind: &dyn Shell) {
        actions::init(self, shell_kind)
    }

    pub fn execute_shell_command(&mut self, shell_kind: &dyn Shell, command: &str) {
        actions::execute_shell_command(self, shell_kind, command)
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

    pub fn show(&mut self, target: Target) {
        let saucefile = self.saucefile();
        actions::show(self, target, saucefile);
    }

    pub fn clear(&mut self, shell_kind: &dyn Shell) {
        let saucefile = self.saucefile();
        actions::clear(self, shell_kind, saucefile);
    }

    pub fn execute(&mut self, shell_kind: &dyn Shell, autoload: bool) {
        let saucefile = self.saucefile();
        actions::execute(self, shell_kind, saucefile, autoload);
    }

    pub fn cascade_paths(&self) -> impl Iterator<Item = PathBuf> {
        self.sauce_path
            .ancestors()
            .filter(|p| {
                if self.data_dir.with_extension("toml") == *p {
                    true
                } else {
                    p.strip_prefix(&self.data_dir).is_ok()
                }
            })
            .map(|p| p.with_extension("toml"))
            .collect::<Vec<PathBuf>>()
            .into_iter()
            .rev()
    }

    pub fn set_var<T: AsRef<str>>(&mut self, values: &[T]) {
        let mut saucefile = self.saucefile();
        for values in values.iter() {
            let mut parts = values.as_ref().splitn(2, '=');
            let var = parts.next().unwrap_or("");
            let value = parts.next().unwrap_or("");

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
            let mut parts = values.as_ref().splitn(2, '=');
            let var = parts.next().unwrap_or("");
            let value = parts.next().unwrap_or("");
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
            data_dir: PathBuf::new(),
            path: PathBuf::new(),
            sauce_path: PathBuf::new(),
            output: Output::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cascade_paths {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_home() {
            let mut context = Context::default();
            context.data_dir = "~/.local/share/sauce".into();
            context.sauce_path = "~/.local/share/sauce".into();

            let paths: Vec<_> = context.cascade_paths().collect();

            let expected: Vec<PathBuf> = vec!["~/.local/share/sauce.toml".into()];
            assert_eq!(paths, expected);
        }

        #[test]
        fn test_nested_subdir() {
            let mut context = Context::default();
            context.data_dir = "~/.local/share/sauce".into();
            context.sauce_path = "~/.local/share/sauce/meow/meow/kitty.toml".into();

            let paths: Vec<_> = context.cascade_paths().collect();

            let expected: Vec<PathBuf> = vec![
                "~/.local/share/sauce.toml".into(),
                "~/.local/share/sauce/meow.toml".into(),
                "~/.local/share/sauce/meow/meow.toml".into(),
                "~/.local/share/sauce/meow/meow/kitty.toml".into(),
            ];
            assert_eq!(paths, expected);
        }
    }
}
