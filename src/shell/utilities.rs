use crate::{
    cli::shape::ShellName,
    shell::kinds::{Bash, Fish, Zsh},
};
use std::env;

use crate::shell::Shell;

/// Ensure proper quoting of any `value` being output to the containing shell.
pub fn escape(value: &str) -> String {
    snailquote::escape(&value.to_string()).replace("\\n", "\n")
}

pub fn get_binary() -> String {
    clap::crate_name!().to_string()
}

pub fn qualify_binary_path(binary: &str) -> String {
    let prefix = if cfg!(feature = "dev") {
        std::env::current_dir()
            .unwrap()
            .join("target/debug/")
            .to_string_lossy()
            .to_string()
    } else {
        "".to_string()
    };
    format!("{}{}", prefix, binary)
}

pub fn detect(shell_name: ShellName) -> Box<dyn Shell> {
    match shell_name {
        ShellName::Zsh => Box::new(Zsh {}),
        ShellName::Fish => Box::new(Fish {}),
        ShellName::Bash => Box::new(Bash {}),
    }
}
