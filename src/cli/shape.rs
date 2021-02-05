use crate::shell::{ColorStrategy, ShellName};
use clap::Clap;
use std::io::Write;

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

    /// Only **show** the intended output of the command rather than executing it.
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
    Shell(ShellCommand),
    Set(SetCommand),
    Config(ConfigCommand),
    New,
    Edit,
    Show(ShowCommand),
    Clear,
}

/// Adds to the sauce file
#[derive(Clap, Debug)]
pub struct ShellCommand {
    #[clap(subcommand)]
    pub kind: ShellKinds,
}

#[derive(Clap, Debug)]
pub enum ShellKinds {
    Init,
    Exec(ExecCommand),
}

/// Sets to the sauce file
#[derive(Clap, Debug)]
pub struct ExecCommand {
    /// The command to run
    #[clap()]
    pub command: String,
}

/// Sets to the sauce file
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

/// Display the given category of key-value pairs
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
