use clap::Clap;
use std::{io::Write, str::FromStr};

/// Sauce!
#[derive(Clap, Debug)]
#[clap(version, author)]
pub struct CliOptions {
    /// Determines the shell behavior, this flag should always be set automatically
    /// by the shell hook. Valid options are: zsh, fish, bash
    #[clap(long)]
    pub shell: ShellName,

    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    pub config: Option<String>,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,

    /// The path which should be sauce'd. Defaults to the current directory.
    #[clap(short, long)]
    pub path: Option<String>,

    /// Runs the given command "as" the given "as" namespace.
    #[clap(short, long)]
    pub r#as: Option<String>,

    /// Filters the set of values to load, allowing globs. By default filters apply to
    /// all targets, but also can use the form "<target>:<glob>" to be more specific.
    #[clap(short, long)]
    pub glob: Option<String>,

    /// Filters the set of values to load, literally. By default filters apply to all
    /// targets, but also can use the form "<target>:<filter>" to be more specific.
    #[clap(short, long)]
    pub filter: Option<String>,

    /// Supplied during autoload sequence. Not generally useful to end-users.
    #[clap(long)]
    pub autoload: bool,

    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

impl CliOptions {
    pub fn parse() -> Self {
        CliOptions::try_parse().unwrap_or_else(|e| {
            let stderr = std::io::stderr();
            let mut handle = stderr.lock();

            let message = format!("{}", e);
            handle.write_all(message.as_ref()).unwrap();
            handle.flush().unwrap();
            std::process::exit(1)
        })
    }
}

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

#[derive(Clap, Debug)]
pub enum SubCommand {
    Shell(ShellCommand),
    Set(SetCommand),
    Config(ConfigCommand),
    New,
    Edit,
    Show,
    Clear,
}

/// Adds to the sauce file
#[derive(Clap, Debug)]
pub struct ShellCommand {
    /// the kind of thing to add
    #[clap(subcommand)]
    pub kind: ShellKinds,
}

#[derive(Clap, Debug)]
pub enum ShellKinds {
    Init,
}

/// Sets to the sauce file
#[derive(Clap, Debug)]
pub struct SetCommand {
    /// the kind of thing to set
    #[clap(subcommand)]
    pub kind: SetKinds,
}

#[derive(Clap, Debug)]
pub enum SetKinds {
    Var(SetVarKind),
    Alias(SetVarKind),
    Function(KeyValuePair),
}

/// Key-value pairs, delimited by an "=".
#[derive(Clap, Debug)]
pub struct SetVarKind {
    pub values: Vec<String>,
}

/// Key value pair, supplied as individual arguments
#[derive(Clap, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

#[derive(Clap, Debug)]
pub struct ConfigCommand {
    #[clap(long, short)]
    pub global: bool,

    #[clap(parse(try_from_str = crate::cli::utilities::parse_key_val))]
    pub values: Vec<(String, String)>,
}
