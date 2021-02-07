use crate::shell::kinds::{Bash, Fish, Zsh};
use std::{env, str::FromStr};

use crate::shell::Shell;

#[derive(Debug)]
pub enum ShellName {
    Zsh,
    Fish,
    Bash,
}

impl FromStr for ShellName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "zsh" => Ok(Self::Zsh),
            "bash" => Ok(Self::Bash),
            "fish" => Ok(Self::Fish),
            unhandled => Err(format!(
                "Unrecognized shell '{}'. Valid options are: zsh, fish, bash",
                unhandled
            )),
        }
    }
}

#[derive(Debug)]
pub enum ColorStrategy {
    Always,
    Never,
    Auto,
}

impl FromStr for ColorStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            "auto" => Ok(Self::Auto),
            unhandled => Err(format!(
                "Unrecognized color value '{}'. Valid options are: always, never, auto",
                unhandled
            )),
        }
    }
}

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

pub fn should_be_colored(strategy: ColorStrategy) -> bool {
    match strategy {
        ColorStrategy::Always => true,
        ColorStrategy::Never => false,
        ColorStrategy::Auto => {
            if atty::is(atty::Stream::Stderr) {
                // NO_COLOR being None implies it should be colored, i.e. true
                std::env::var_os("NO_COLOR").is_none()
            } else {
                false
            }
        }
    }
}
