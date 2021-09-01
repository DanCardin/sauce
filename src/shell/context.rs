use ansi_term::ANSIString;
use anyhow::Result;
use core::iter::FromIterator;
use indexmap::IndexSet;
use itertools::Itertools;
use path_absolutize::Absolutize;
use std::path::Path;
use std::path::PathBuf;
use toml_edit::Item;

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

// ~/ to ~/foo/bar:
//  - on-exit: n/a
//  - on-enter: ~/foo, ~/foo/bar
//
// ~/meow/bar to ~/foo/bar:
//  - on-exit: ~/meow/bar, ~/meow
//  - on-enter: ~/foo, ~/foo/bar
//
// ~/foo to ~/foo/bar:
//  - on-exit: n/a
//  - on-enter: ~/foo/bar
//
// ~/foo/bar to ~/:
//  - on-exit: ~/foo/bar, ~/foo
//  - on-enter: n/a
fn diff_paths<T>(corpus: &corpus::Corpus, from_paths: T, to_paths: T) -> Vec<PathBuf>
where
    T: IntoIterator<Item = PathBuf>,
{
    let from_paths_set: IndexSet<PathBuf> = IndexSet::from_iter(from_paths);
    let to_paths_set: IndexSet<PathBuf> = IndexSet::from_iter(to_paths);

    let difference = to_paths_set.difference(&from_paths_set);
    difference.into_iter().map(|x| x.clone()).collect()
}

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
            self._saucefile = Some(Saucefile::read_with_ancestors(
                output,
                &self.sauce_path(),
                self.cascade_paths(self.path.as_path()),
            ));
        }
    }

    fn saucefile(&self) -> &Saucefile {
        self._saucefile.as_ref().unwrap()
    }

    fn saucefile_mut(&mut self) -> &mut Saucefile {
        self._saucefile.as_mut().unwrap()
    }

    pub fn with_sauce_path(&mut self, sauce_path: PathBuf) {
        self._sauce_path = Some(sauce_path);
    }

    pub fn with_settings(&mut self, settings: Settings) {
        self._settings = Some(settings);
    }

    pub fn with_corpus(&mut self, corpus: corpus::Corpus) {
        self.corpus = corpus;
    }

    pub fn at_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.path = path.into();
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

    pub fn create_saucefile(&mut self, output: &mut Output) {
        actions::create_saucefile(output, &self.corpus.path(self.sauce_path().as_path()));
    }

    pub fn move_saucefile(&self, output: &mut Output, destination: &Path, copy: bool) {
        let source = &self.corpus.path(self.sauce_path().as_path());

        let destination = match destination.absolutize() {
            Ok(d) => d,
            Err(_) => {
                output.notify_error(
                    ErrorCode::WriteError,
                    &[RED.paint("Path is not relative to the home directory")],
                );
                return;
            }
        };
        if let Ok(relative_path) = destination.strip_prefix(&self.corpus.relative_path) {
            let dest = self.corpus.path(relative_path);
            actions::move_saucefile(output, source, &dest, copy);
        } else {
            output.notify_error(
                ErrorCode::WriteError,
                &[RED.paint("Path is not relative to the home directory")],
            );
        }
    }

    pub fn edit_saucefile(&mut self, shell_kind: &dyn Shell, output: &mut Output) {
        actions::edit(output, shell_kind, self.sauce_path().as_path());
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

    pub fn execute(
        &mut self,
        shell_kind: &dyn Shell,
        autoload: Option<&PathBuf>,
        output: &mut Output,
    ) {
        self.load_saucefile(output);
        self.load_settings(output);

        // The `autoload_flag` indicates that the "context" of the execution is happening during
        // an autoload, i.e. `cd`. It's the precondition for whether we need to check the settings to
        // see whether we **actually** should perform the autoload, or exit early.
        if let Some(previous_location) = autoload {
            let current_paths = self.cascade_paths(&self.path.as_path());
            let previous_paths = self.cascade_paths(&previous_location.as_path());
            let forward_paths = dbg!(diff_paths(&self.corpus, current_paths, previous_paths));
            let saucefile =
                Saucefile::read_with_ancestors(output, &self.sauce_path(), forward_paths);

            let current_paths = self.cascade_paths(&self.path.as_path());
            let previous_paths = self.cascade_paths(&previous_location.as_path());
            let backward_paths = dbg!(diff_paths(&self.corpus, previous_paths, current_paths));
            let prev_saucefile =
                Saucefile::read_with_ancestors(output, &self.sauce_path(), backward_paths);

            // let should_continue =
            //     actions::autoload(output, self.settings(), &prev_saucefile, &saucefile);
            // if !should_continue {
            //     return;
            // }
        } else {
            // Saucefile::read_with_ancestors(
            //     output,
            //     &self.sauce_path(),
            //     paths,
            // ))
        }

        // let message =
        //     materialize_path_message("Sauced", &self.corpus.root_location, saucefile.paths());
        // output.notify(&message);
    }

    pub fn cascade_paths(&self, path: &Path) -> impl Iterator<Item = PathBuf> {
        if let Some(path) = &self._sauce_path {
            vec![path.clone()].into_iter().rev()
        } else {
            dbg!(self
                .corpus
                .ancestors(path)
                .collect::<Vec<PathBuf>>()
                .into_iter()
                .rev())
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

            let mut context = Context::default();
            context.with_corpus(corpus(home.join(".local/share").as_path()));
            context.at_path(&home);

            let paths: Vec<_> = context
                .cascade_paths(&Path::new("~/.local/share/sauce").to_path_buf())
                .collect();

            let expected: Vec<PathBuf> = vec![home.join(".local/share/sauce.toml")];
            assert_eq!(paths, expected);
        }

        #[test]
        fn test_nested_subdir() {
            let home = etcetera::home_dir().unwrap();

            let mut context = Context::default();
            context.with_corpus(corpus(home.join(".local/share").as_path()));
            context.at_path(home.join("meow/meow/kitty"));

            let paths: Vec<_> = context
                .cascade_paths(
                    &Path::new("~/.local/share/sauce/meow/meow/kitty.toml").to_path_buf(),
                )
                .collect();

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
