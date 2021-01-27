use crate::shell::kinds::{Bash, Zsh};
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
    let prefix = if cfg!(dev) {
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

pub fn detect() -> Box<dyn Shell> {
    if std::env::var_os("ZSH_VERSION").is_some() {
        return Box::new(Zsh {});
    }

    if std::env::var_os("BASH_VERSION").is_some() {
        return Box::new(Bash {});
    }

    return Box::new(Bash {});
}
