use crate::shell::{ColorStrategy, ShellName};
use clap::Clap;
use std::{io::Write, path::PathBuf};

/// Sauce!
#[derive(Clap, Debug)]
#[clap(version, author)]
pub struct CliOptions {
    /// Determines the shell behavior, this flag should always be set automatically
    /// by the shell hook. Valid options are: zsh, fish, bash
    #[clap(long)]
    pub shell: ShellName,

    /// Supplied during autoload sequence. Not generally useful to end-users.
    #[clap(long)]
    pub autoload: bool,

    /// For typical commands such as `sauce` and `sauce clear` this outputs the exact
    /// shell output that would have executed. For mutating commands like `sauce config`
    /// and `sauce set`, the change is printed but not saved.
    #[clap(long)]
    pub show: bool,

    /// Valid options: always, never, auto. Auto (default) will attempt to autodetect
    /// whether it should output color based on the existence of a tty.
    #[clap(long, default_value = "auto")]
    pub color: ColorStrategy,

    /// Enables verbose output. This causes all stdout to be mirrored to stderr.
    #[clap(short, long)]
    pub verbose: bool,

    /// Disables normal messaging output after a command is executed.
    #[clap(short, long)]
    pub quiet: bool,

    /// The path which should be sauce'd. Defaults to the current directory.
    #[clap(short, long)]
    pub path: Option<String>,

    /// Sets a specific saucefile to load, overriding the default lookup mechanisms and
    /// cascading behavior
    #[clap(long)]
    pub file: Option<String>,

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

#[derive(Clap, Debug)]
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

#[derive(Clap, Debug)]
pub struct ConfigCommand {
    #[clap(long, short)]
    pub global: bool,

    #[clap(parse(try_from_str = crate::cli::utilities::parse_key_val))]
    pub values: Vec<(String, String)>,
}

#[derive(Clap, Debug)]
pub struct MoveCommand {
    /// The destination location to which a `sauce` invocation would point.
    /// That is, not the destination saucefile location.
    #[clap()]
    pub destination: PathBuf,

    /// Instead of removing the files at the source location, leave the original
    /// file untouched.
    #[clap(short, long)]
    pub copy: bool,
}

#[derive(Clap, Debug)]
pub struct SetCommand {
    #[clap(subcommand)]
    pub kind: SetKinds,
}

#[derive(Clap, Debug)]
pub enum SetKinds {
    Env(SetVarKind),
    Alias(SetVarKind),
    Function(KeyValuePair),
}

/// Key-value pairs, delimited by an "=".
#[derive(Clap, Debug)]
pub struct SetVarKind {
    #[clap(parse(try_from_str = crate::cli::utilities::parse_key_val))]
    pub values: Vec<(String, String)>,
}

/// Key value pair, supplied as individual arguments
#[derive(Clap, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

#[derive(Clap, Debug)]
pub struct ShellCommand {
    #[clap(subcommand)]
    pub kind: ShellKinds,
}

#[derive(Clap, Debug)]
pub enum ShellKinds {
    /// The intialization shell hook for getting sauce functionality
    Init,

    /// Executes a command inside a subshell which has had `sauce` invoked already
    Exec(ExecCommand),
}

#[derive(Clap, Debug)]
pub struct ExecCommand {
    #[clap()]
    pub command: String,
}

#[derive(Clap, Debug)]
pub struct ShowCommand {
    #[clap(subcommand)]
    pub kind: ShowKinds,
}

#[derive(Clap, Debug)]
pub enum ShowKinds {
    Env,
    Alias,
    Function,
}
