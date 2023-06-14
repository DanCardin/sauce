use crate::shell::{ColorStrategy, ShellName};
use clap::Parser;
use std::{io::Write, path::PathBuf};

/// Sauce!
#[derive(Parser, Debug)]
#[command(version, author)]
pub struct CliOptions {
    /// Determines the shell behavior, this flag should always be set automatically
    /// by the shell hook. Valid options are: zsh, fish, bash
    #[arg(long)]
    pub shell: ShellName,

    /// Supplied during autoload sequence. Not generally useful to end-users.
    #[arg(long)]
    pub autoload: bool,

    /// For typical commands such as `sauce` and `sauce clear` this outputs the exact
    /// shell output that would have executed. For mutating commands like `sauce config`
    /// and `sauce set`, the change is printed but not saved.
    #[arg(long)]
    pub show: bool,

    /// Valid options: always, never, auto. Auto (default) will attempt to autodetect
    /// whether it should output color based on the existence of a tty.
    #[arg(long, default_value = "auto")]
    pub color: ColorStrategy,

    /// Enables verbose output. This causes all stdout to be mirrored to stderr.
    #[arg(short, long)]
    pub verbose: bool,

    /// Disables normal messaging output after a command is executed.
    #[arg(short, long)]
    pub quiet: bool,

    /// The path which should be sauce'd. Defaults to the current directory.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Sets a specific saucefile to load, overriding the default lookup mechanisms and
    /// cascading behavior
    #[arg(long)]
    pub file: Option<PathBuf>,

    /// Runs the given command "as" the given "as" namespace.
    #[arg(short, long)]
    pub r#as: Option<Vec<String>>,

    /// Filters the set of values to load, allowing globs. By default filters apply to
    /// all targets, but also can use the form "<target>:<glob>" to be more specific.
    #[arg(short, long)]
    pub glob: Option<String>,

    /// Only use values for a specific target. Essentially this can be thought of
    /// as a shortcut for `-g '<target>:*`.
    #[arg(short, long)]
    pub target: Option<String>,

    /// Filters the set of values to load, literally. By default filters apply to all
    /// targets, but also can use the form "<target>:<filter>" to be more specific.
    #[arg(short, long)]
    pub filter: Option<String>,

    #[command(subcommand)]
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

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Clears the shell of values sauce tracks
    Clear,

    /// Sets local/global configuration options
    Config(ConfigCommand),

    /// Opens the saucefile with your $EDITOR
    Edit,

    /// Moves the targeted saucefile to the location given by `destination`.
    Move(MoveCommand),

    /// Creates a new saucefile for the targeted location
    New,

    /// Sets target values for the targeted location
    Set(SetCommand),

    /// Group of shell related subcommands
    Shell(ShellCommand),

    /// Display the given category of key-value pairs
    Show(ShowCommand),
}

#[derive(Parser, Debug)]
pub struct ConfigCommand {
    #[arg(long, short)]
    pub global: bool,

    #[arg(value_parser = crate::cli::utilities::parse_key_val::<String>)]
    pub values: Vec<(String, String)>,
}

#[derive(Parser, Debug)]
pub struct MoveCommand {
    /// The destination location to which a `sauce` invocation would point.
    /// That is, not the destination saucefile location.
    #[arg()]
    pub destination: PathBuf,

    /// Instead of removing the files at the source location, leave the original
    /// file untouched.
    #[arg(short, long)]
    pub copy: bool,
}

#[derive(Parser, Debug)]
pub struct SetCommand {
    #[command(subcommand)]
    pub kind: SetKinds,
}

#[derive(Parser, Debug)]
pub enum SetKinds {
    Env(SetVarKind),
    Alias(SetVarKind),
    Function(KeyValuePair),
    File(KeyValuePair),
}

/// Key-value pairs, delimited by an "=".
#[derive(Parser, Debug)]
pub struct SetVarKind {
    #[arg(value_parser = crate::cli::utilities::parse_key_val::<String>)]
    pub values: Vec<(String, String)>,
}

/// Key value pair, supplied as individual arguments
#[derive(Parser, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

#[derive(Parser, Debug)]
pub struct ShellCommand {
    #[command(subcommand)]
    pub kind: ShellKinds,
}

#[derive(Parser, Debug)]
pub enum ShellKinds {
    /// The intialization shell hook for getting sauce functionality
    Init,

    /// Executes a command inside a subshell which has had `sauce` invoked already
    Exec(ExecCommand),
}

#[derive(Parser, Debug)]
pub struct ExecCommand {
    #[arg()]
    pub command: String,
}

#[derive(Parser, Debug)]
pub struct ShowCommand {
    #[command(subcommand)]
    pub kind: ShowKinds,
}

#[derive(Parser, Debug)]
pub enum ShowKinds {
    Env,
    Alias,
    Function,
    File,
}
