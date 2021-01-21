use crate::shell::kinds::{Bash, Zsh};
use std::{env, path::Path};

use crate::shell::Shell;

pub fn escape(value: &str) -> String {
    snailquote::escape(&value.to_string()).replace("\\n", "\n")
}

pub fn get_binary() -> String {
    clap::crate_name!().to_string()
}

pub fn detect() -> Box<dyn Shell> {
    let shell = std::env::var_os("SHELL");
    let shell = shell
        .as_ref()
        .and_then(|s| Path::new(s).file_stem())
        .and_then(|s| s.to_str());

    match shell {
        Some("zsh") => Box::new(Zsh {}),
        _ => Box::new(Bash {}),
    }
}
