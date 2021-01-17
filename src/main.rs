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

    /// The path which should be sauce'd. Defaults to the current directory.
    #[clap(short, long)]
    path: Option<String>,

    /// Runs the given command "as" the given "as" namespace.
    #[clap(short, long)]
    r#as: Option<String>,

    /// Filters the set of values to load, allowing globs. By default filters apply to
    /// all targets, but also can use the form "<target>:<glob>" to be more specific.
    #[clap(short, long)]
    glob: Option<String>,

    /// Filters the set of values to load, literally. By default filters apply to all
    /// targets, but also can use the form "<target>:<filter>" to be more specific.
    #[clap(short, long)]
    filter: Option<String>,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap, Debug)]
enum SubCommand {
    New(NewCommand),
    Set(SetCommand),
    Shell(ShellCommand),
    Edit,
    Show,
    Clear,
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

    let options = GlobalOptions::new(
        opts.glob.as_deref(),
        opts.filter.as_deref(),
        opts.r#as.as_deref(),
        opts.path.as_deref(),
    );

    let context = Context::new(&options)?;

    let shell_kind = &*shell::detect();

    let mut output = Output::default();
    match opts.subcmd {
        Some(SubCommand::New(cmd)) => crate::commands::new::new(context, cmd, &mut output),
        Some(SubCommand::Set(cmd)) => crate::commands::set::set(context, cmd, &mut output),
        Some(SubCommand::Shell(cmd)) => crate::commands::shell::run(shell_kind, cmd, &mut output),
        Some(SubCommand::Edit) => shell::actions::edit(shell_kind, context, &mut output),
        Some(SubCommand::Show) => shell::actions::show(shell_kind, context, &mut output, &options),
        Some(SubCommand::Clear) => {
            shell::actions::clear(shell_kind, context, &mut output, &options)
        }
        None => shell::actions::execute(shell_kind, context, &mut output, &options),
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
