mod commands;
mod context;
mod option;
mod output;
mod saucefile;
mod shell;

use crate::commands::new::NewCommand;
use crate::commands::set::SetCommand;
use crate::commands::shell::ShellCommand;
use crate::context::Context;
use crate::option::GlobalOptions;
use crate::shell::Shell;
use anyhow::Result;
use output::Output;
use std::io::Write;

use clap::Clap;

/// Sauce!
#[derive(Clap, Debug)]
#[clap(version, author)]
struct Options {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    config: Option<String>,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    /// Runs the given command "as" the given "as" namespace.
    #[clap(short, long)]
    r#as: Option<String>,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap, Debug)]
enum SubCommand {
    New(NewCommand),
    Set(SetCommand),
    Shell(ShellCommand),
    Edit(CommandPath),
    Show,
    Clear,
}

/// Sets to the sauce file
#[derive(Clap, Debug)]
pub struct CommandPath {
    /// the kind of thing to set
    #[clap(default_value = ".")]
    path: String,
}

fn main() -> Result<()> {
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();

    let opts: Options = Options::try_parse().unwrap_or_else(|e| {
        let message = format!("{}", e);
        handle.write_all(message.as_ref()).unwrap();
        handle.flush().unwrap();
        std::process::exit(1)
    });

    let context = Context::from_current_dir()?;
    let mut output = Output::default();

    let options = GlobalOptions::new(opts.r#as.as_deref(), &[], &[]);

    match opts.subcmd {
        Some(SubCommand::New(cmd)) => crate::commands::new::new(context, cmd, &mut output),
        Some(SubCommand::Set(cmd)) => crate::commands::set::set(context, cmd, &mut output),
        Some(SubCommand::Shell(cmd)) => crate::commands::shell::run(context, cmd, &mut output),
        Some(SubCommand::Edit(cmd)) => {
            let context = Context::from_path(cmd.path)?;
            Shell::new(context).edit(&mut output);
        }
        Some(SubCommand::Show) => Shell::new(context).show(&mut output, options),
        Some(SubCommand::Clear) => Shell::new(context).clear(&mut output, options),
        None => Shell::new(context).execute(&mut output, options),
    };

    let out = std::io::stderr();
    let mut handle = out.lock();
    handle.write_all(output.message().as_ref())?;
    handle.flush()?;

    let out = std::io::stdout();
    let mut handle = out.lock();
    handle.write_all(output.result().as_ref())?;
    handle.flush()?;

    Ok(())
}
