use anyhow::Result;
use etcetera::home_dir;
use std::path::PathBuf;
use std::{env, path::Path};

use crate::{
    colors::{BLUE, RED, YELLOW},
    filter::FilterOptions,
    output::{ErrorCode, Output},
    saucefile::Saucefile,
    settings::Settings,
    shell::{actions, Shell},
    target::Target,
    toml::value_from_string,
};

#[derive(Debug)]
pub struct Context<'a> {
    pub filter_options: FilterOptions<'a>,

    pub data_dir: PathBuf,
    pub config_dir: PathBuf,

    pub path: PathBuf,
    pub sauce_path: PathBuf,

    _settings: Option<Settings>,
    _saucefile: Option<Saucefile>,
}

impl<'a> Context<'a> {
    pub fn new(
        data_dir: PathBuf,
        config_dir: PathBuf,
        filter_options: FilterOptions<'a>,
        path: Option<&'a str>,
        file: Option<&'a str>,
    ) -> Result<Self> {
        let (path, sauce_path, data_dir) = match file {
            // The default case, where no `file` is supplied. We perform normal
            // path lookup and saucefile cascading behavior.
            None => {
                let path = if let Some(path) = path {
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
            config_dir,
            filter_options,
            path,
            sauce_path,
            _saucefile: None,
            _settings: None,
        })
    }

    fn load_saucefile(&mut self, output: &mut Output) {
        if self._saucefile.is_none() {
            self._saucefile = Some(Saucefile::read(self, output));
        }
    }

    fn saucefile(&self) -> &Saucefile {
        self._saucefile.as_ref().unwrap()
    }

    fn saucefile_mut(&mut self) -> &mut Saucefile {
        self._saucefile.as_mut().unwrap()
    }

    pub fn set_settings(&mut self, settings: Settings) {
        self._settings = Some(settings);
    }

    pub fn load_settings(&mut self, output: &mut Output) {
        if self._settings.is_none() {
            self._settings = Some(Settings::load(&self.config_dir, output));
        }
    }

    pub fn settings(&self) -> &Settings {
        self._settings.as_ref().unwrap()
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        self._settings.as_mut().unwrap()
    }

    pub fn init_shell(&mut self, shell_kind: &dyn Shell, output: &mut Output) {
        self.load_settings(output);
        let autoload_hook = self.settings().autoload_hook.unwrap_or(false);
        actions::init(shell_kind, output, autoload_hook)
    }

    pub fn execute_shell_command(
        &mut self,
        shell_kind: &dyn Shell,
        command: &str,
        output: &mut Output,
    ) {
        actions::execute_shell_command(shell_kind, command, output)
    }

    pub fn create_saucefile(&mut self, output: &mut Output) {
        let parent = self.sauce_path.parent().unwrap();
        if std::fs::create_dir_all(parent).is_err() {
            output.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.paint("Couldn't create "),
                    YELLOW.paint(parent.to_string_lossy()),
                ],
            );
            return;
        }

        if self.sauce_path.is_file() {
            output.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.bold().paint("File already exists at "),
                    YELLOW.paint(self.sauce_path.to_string_lossy()),
                ],
            );
        } else if std::fs::File::create(&self.sauce_path).is_err() {
            output.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.bold().paint("Couldn't create"),
                    YELLOW.paint(self.sauce_path.to_string_lossy()),
                ],
            );
        } else {
            output.notify(&[
                BLUE.bold().paint("Created"),
                YELLOW.paint(self.sauce_path.to_string_lossy()),
            ]);
        }
    }

    pub fn edit_saucefile(&mut self, shell_kind: &dyn Shell, output: &mut Output) {
        actions::edit(shell_kind, output, &self.sauce_path);
    }

    pub fn show(&mut self, target: Target, output: &mut Output) {
        self.load_saucefile(output);
        actions::show(&self.filter_options, target, self.saucefile(), output);
    }

    pub fn clear(&mut self, shell_kind: &dyn Shell, output: &mut Output) {
        self.load_settings(output);
        self.load_saucefile(output);

        actions::clear(
            shell_kind,
            self.saucefile(),
            self.settings(),
            &self.filter_options,
            output,
        );
    }

    pub fn execute(&mut self, shell_kind: &dyn Shell, autoload: bool, output: &mut Output) {
        self.load_saucefile(output);
        self.load_settings(output);
        actions::execute(
            shell_kind,
            self.saucefile(),
            self.settings(),
            &self.filter_options,
            autoload,
            output,
        );
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

    pub fn set_var<T: AsRef<str>>(&mut self, raw_values: &[(T, T)], output: &mut Output) {
        self.load_saucefile(output);

        let values = raw_values
            .iter()
            .map(|(name, raw_value)| (name, value_from_string(raw_value.as_ref())))
            .collect::<Vec<_>>();

        let path = self.sauce_path.clone();
        output.write_toml(
            &path,
            &mut self.saucefile_mut().document,
            "environment",
            values,
        );
    }

    pub fn set_alias<T: AsRef<str>>(&mut self, raw_values: &[(T, T)], output: &mut Output) {
        self.load_saucefile(output);

        let values = raw_values
            .iter()
            .map(|(name, raw_value)| (name, value_from_string(raw_value.as_ref())))
            .collect::<Vec<_>>();

        let path = self.sauce_path.clone();
        output.write_toml(&path, &mut self.saucefile_mut().document, "alias", values);
    }

    pub fn set_function(&mut self, name: &str, body: &str, output: &mut Output) {
        self.load_saucefile(output);
        let values = vec![(name, value_from_string(body))];

        let path = self.sauce_path.clone();
        output.write_toml(
            &path,
            &mut self.saucefile_mut().document,
            "function",
            values,
        );
    }

    pub fn set_config<T: AsRef<str>>(
        &mut self,
        values: &[(T, T)],
        global: bool,
        output: &mut Output,
    ) {
        if global {
            self.load_settings(output);
            self.settings_mut().set_values(&values, output);
        } else {
            self.load_saucefile(output);
            let settings = self.saucefile().settings();
            settings.set_values(&values, output);
        };
    }
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self {
            filter_options: FilterOptions::default(),
            data_dir: PathBuf::new(),
            config_dir: PathBuf::new(),
            path: PathBuf::new(),
            sauce_path: PathBuf::new(),
            _saucefile: None,
            _settings: None,
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
