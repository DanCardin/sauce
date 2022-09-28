use ansi_term::ANSIString;
use anyhow::Result;
use itertools::Itertools;
use std::path::Path;
use std::path::PathBuf;
use toml_edit::Item;

use crate::{
    colors::{BLUE, RED, YELLOW},
    filter::FilterOptions,
    output::Output,
    saucefile::Saucefile,
    settings::Settings,
    shell::{actions, Shell},
    target::Target,
    toml::value_from_string,
};

#[derive(Debug)]
pub struct Context<'a> {
    filter_options: FilterOptions<'a>,

    corpus: corpus::Corpus,
    config_dir: PathBuf,
    path: PathBuf,

    _sauce_path: Option<PathBuf>,
    _settings: Option<Settings>,
    _saucefile: Option<Saucefile>,
}

impl<'a> Context<'a> {
    pub fn new(
        config_dir: PathBuf,
        filter_options: FilterOptions<'a>,
        path: Option<&'a Path>,
        file: Option<&'a Path>,
    ) -> Result<Self> {
        let builder = corpus::builder().relative_to_home()?;

        let corpus = builder
            .with_root(corpus::RootLocation::XDGData)
            .with_name("sauce")
            .with_extension("toml")
            .build()?;

        let path = if let Some(p) = path {
            p.to_path_buf()
        } else {
            std::env::current_dir()?
        };

        Ok(Self {
            corpus,
            config_dir,
            filter_options,
            path,
            _sauce_path: file.map(|p| p.to_path_buf()),
            _saucefile: None,
            _settings: None,
        })
    }

    fn sauce_path(&self) -> PathBuf {
        if let Some(path) = &self._sauce_path {
            path.to_path_buf()
        } else {
            self.corpus.path(self.path.as_path())
        }
    }

    fn load_saucefile(&mut self, output: &mut Output) {
        if self._saucefile.is_none() {
            self._saucefile = Some(Saucefile::read(output, self.cascade_paths()));
        }
    }

    fn saucefile(&self) -> &Saucefile {
        self._saucefile.as_ref().unwrap()
    }

    fn saucefile_mut(&mut self) -> &mut Saucefile {
        self._saucefile.as_mut().unwrap()
    }

    pub fn with_sauce_path(mut self, sauce_path: PathBuf) -> Self {
        self._sauce_path = Some(sauce_path);
        self
    }

    pub fn with_settings(mut self, settings: Settings) -> Self {
        self._settings = Some(settings);
        self
    }

    pub fn with_corpus(mut self, corpus: corpus::Corpus) -> Self {
        self.corpus = corpus;
        self
    }

    pub fn at_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.path = path.into();
        self
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
        actions::init(output, shell_kind, autoload_hook)
    }

    pub fn execute_shell_command(
        &mut self,
        shell_kind: &dyn Shell,
        command: &str,
        output: &mut Output,
    ) {
        actions::execute_shell_command(output, shell_kind, command)
    }

    pub fn create_saucefile(&self, output: &mut Output) {
        output.create_file(&self.sauce_path()).ok();
    }

    pub fn move_saucefile(&self, output: &mut Output, destination: &Path, copy: bool) {
        let dest = self.corpus.path(destination);
        output.move_file(&self.sauce_path(), &dest, copy).ok();
    }

    pub fn edit_saucefile(&mut self, shell_kind: &dyn Shell, output: &mut Output) {
        let path = self.sauce_path();
        if !path.is_file() {
            self.create_saucefile(output);
        }
        actions::edit(output, shell_kind, &path);
    }

    pub fn show(&mut self, target: Target, output: &mut Output) {
        self.load_saucefile(output);
        actions::show(output, &self.filter_options, target, self.saucefile());
    }

    pub fn clear(&mut self, shell_kind: &dyn Shell, output: &mut Output) {
        self.load_settings(output);
        self.load_saucefile(output);

        actions::clear(
            output,
            shell_kind,
            self.saucefile(),
            self.settings(),
            &self.filter_options,
        );
    }

    pub fn execute(&mut self, shell_kind: &dyn Shell, autoload: bool, output: &mut Output) {
        self.load_saucefile(output);
        self.load_settings(output);

        let saucefile = self.saucefile();
        let sauced = actions::execute(
            output,
            shell_kind,
            saucefile,
            self.settings(),
            &self.filter_options,
            autoload,
        );

        if !sauced {
            // We may sometimes opt to *not* execute, i.e. certain autoload scenarios.
            return;
        }

        let message =
            materialize_path_message("Sauced", &self.corpus.root_location, saucefile.paths());
        output.notify(&message);
    }

    pub fn cascade_paths(&self) -> impl Iterator<Item = PathBuf> {
        if let Some(path) = &self._sauce_path {
            vec![path.clone()].into_iter().rev()
        } else {
            self.corpus
                .ancestors(self.path.as_path())
                .collect::<Vec<PathBuf>>()
                .into_iter()
                .rev()
        }
    }

    pub fn set_var<T: AsRef<str>>(&mut self, raw_values: &[(T, T)], output: &mut Output) {
        self.load_saucefile(output);

        let values = raw_values
            .iter()
            .map(|(name, raw_value)| (name, value_from_string(raw_value.as_ref())))
            .collect::<Vec<_>>();

        self.set_values(output, "environment", values);
    }

    pub fn set_alias<T: AsRef<str>>(&mut self, raw_values: &[(T, T)], output: &mut Output) {
        self.load_saucefile(output);

        let values = raw_values
            .iter()
            .map(|(name, raw_value)| (name, value_from_string(raw_value.as_ref())))
            .collect::<Vec<_>>();

        self.set_values(output, "alias", values);
    }

    pub fn set_function(&mut self, name: &str, body: &str, output: &mut Output) {
        self.load_saucefile(output);
        let values = vec![(name, value_from_string(body))];

        self.set_values(output, "function", values);
    }

    fn set_values<I, T>(&mut self, output: &mut Output, section: &str, values: I)
    where
        I: IntoIterator<Item = (T, Item)>,
        T: AsRef<str>,
    {
        let path = self.sauce_path();
        let document = &mut self.saucefile_mut().document;

        output.write_toml(&path, document, section, values);
    }

    pub fn set_config<T: AsRef<str>>(
        &mut self,
        values: &[(T, T)],
        global: bool,
        output: &mut Output,
    ) {
        if global {
            self.load_settings(output);
            self.settings_mut().set_values(values, output);
        } else {
            self.load_saucefile(output);
            let settings = self.saucefile().settings();
            settings.set_values(values, output);
        };
    }
}

fn materialize_path_message<'a>(
    action: &'a str,
    data_dir: &'a Path,
    paths: impl Iterator<Item = &'a PathBuf>,
) -> Vec<ANSIString<'a>> {
    let paths = paths
        .filter_map(|p| p.strip_prefix(data_dir).ok())
        .map(|p| p.to_string_lossy())
        .join(", ");

    let mut result = Vec::new();

    if paths.is_empty() {
        result.push(RED.bold().paint("No saucefiles exist"));
        return result;
    }

    result.push(BLUE.bold().paint(format!("{} ", action)));
    result.push(YELLOW.paint(paths.clone()));

    if !paths.starts_with(data_dir.to_string_lossy().as_ref()) {
        result.push(BLUE.bold().paint(" from "));
        result.push(YELLOW.paint(data_dir.to_string_lossy()));
    }
    result
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self {
            filter_options: FilterOptions::default(),
            corpus: corpus::builder().build().unwrap(),
            config_dir: PathBuf::new(),
            path: PathBuf::new(),
            _sauce_path: None,
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

        fn corpus<'a, P: Into<&'a Path>>(root: P) -> corpus::Corpus {
            let root = root.into();
            corpus::builder()
                .relative_to_home()
                .unwrap()
                .with_root(root)
                .with_name("sauce")
                .with_extension("toml")
                .build()
                .unwrap()
        }

        #[test]
        fn test_home() {
            let home = etcetera::home_dir().unwrap();

            let context = Context::default()
                .with_corpus(corpus(home.join(".local/share").as_path()))
                .at_path(&home);

            let paths: Vec<_> = context.cascade_paths().collect();

            let expected: Vec<PathBuf> = vec![home.join(".local/share/sauce.toml")];
            assert_eq!(paths, expected);
        }

        #[test]
        fn test_nested_subdir() {
            let home = etcetera::home_dir().unwrap();

            let context = Context::default()
                .with_corpus(corpus(home.join(".local/share").as_path()))
                .at_path(home.join("meow/meow/kitty"));

            let paths: Vec<_> = context.cascade_paths().collect();

            let expected: Vec<PathBuf> = vec![
                home.join(".local/share/sauce.toml"),
                home.join(".local/share/sauce/meow.toml"),
                home.join(".local/share/sauce/meow/meow.toml"),
                home.join(".local/share/sauce/meow/meow/kitty.toml"),
            ];
            assert_eq!(paths, expected);
        }
    }
}
