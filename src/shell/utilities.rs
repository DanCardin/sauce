use crate::shell::kinds::{Bash, Zsh};
use std::env;

use crate::shell::Shell;

pub fn escape(value: &str) -> String {
    snailquote::escape(&value.to_string()).replace("\\n", "\n")
}

pub fn get_binary() -> String {
    let prefix = if cfg!(debug_assertions) {
        "./target/debug/"
    } else {
        ""
    };

    format!("{}{}", prefix, clap::crate_name!())
}

// enum ShellEnum {
//     Zsh(Zsh),
//     Bash(Bash),
// }
// impl Shell for ShellEnum {}
//
// pub fn detect() -> impl Shell {
//     let shell_name = env::var_os("SHELL").and_then(|s| s.into_string().ok());
//
//     match shell_name.as_deref() {
//         Some("zsh") => ShellEnum::Zsh(Zsh {}),
//         Some("bash") | Some("sh") => ShellEnum::Bash(Bash {}),
//         _ => ShellEnum::Bash(Bash {}),
//     }
// }
pub fn detect() -> Box<dyn Shell> {
    let shell_name = env::var_os("SHELL").and_then(|s| s.into_string().ok());

    match shell_name.as_deref() {
        Some("zsh") => return Box::new(Zsh {}),
        Some("bash") | Some("sh") => Box::new(Bash {}),
        _ => return Box::new(Bash {}),
    }
}
