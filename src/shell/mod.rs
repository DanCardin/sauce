mod actions;
pub mod context;
mod kinds;
mod utilities;
use crate::shell::utilities::escape;

use std::ffi::OsString;

pub use kinds::{Bash, Fish, Zsh};
pub use utilities::{detect, should_be_colored, ColorStrategy, ShellName};

pub trait Shell {
    fn name(&self) -> &'static str;

    fn edit(&self, editor: Option<OsString>, path: &str) -> Option<String> {
        editor.map(|e| format!("{} '{}'", e.to_string_lossy(), path))
    }

    fn init(&self, binary: &str, autoload: bool, default_args: &str) -> String;
    fn set_var(&self, var: &str, value: &str) -> String;
    fn unset_var(&self, var: &str) -> String;

    fn set_alias(&self, var: &str, value: &str) -> String;
    fn unset_alias(&self, var: &str) -> String;

    fn set_function(&self, var: &str, value: &str) -> String;
    fn unset_function(&self, var: &str) -> String;

    fn set_file(&self, var: &str, value: &str) -> String {
        format!("printf '{1}' > {0}", escape(var), value)
    }
    fn unset_file(&self, var: &str) -> String {
        format!("rm '{}'", escape(var))
    }
}
