use crate::shell::kinds::{Bash, Zsh};
use std::env;

use crate::shell::Shell;

pub fn escape(value: &str) -> String {
    snailquote::escape(&value.to_string()).replace("\\n", "\n")
}

pub fn get_binary() -> String {
    clap::crate_name!().to_string()
}

pub fn detect() -> Box<dyn Shell> {
    let shell_name = env::var_os("SHELL").and_then(|s| s.into_string().ok());

    match shell_name.as_deref() {
        Some("zsh") => Box::new(Zsh {}),
        Some("bash") | Some("sh") => Box::new(Bash {}),
        _ => Box::new(Bash {}),
    }
}
