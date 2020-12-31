mod commands;
mod context;
mod output;
mod saucefile;
mod shell;

use crate::commands::new::NewCommand;
use crate::commands::r#as::AsCommand;
use crate::commands::set::SetCommand;
use crate::commands::shell::ShellCommand;
use crate::context::Context;
use crate::shell::Shell;
use anyhow::Result;
use output::Output;
use std::io::Write;

use clap::Clap;

/// Sauce!
#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Dan C. <ddcardin@gmail.com>")]
struct Options {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    config: Option<String>,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap, Debug)]
enum SubCommand {
    New(NewCommand),
    Set(SetCommand),
    Shell(ShellCommand),
    As(AsCommand),
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

    let context = Context::new()?;
    let mut output = Output::default();

    match opts.subcmd {
        Some(SubCommand::New(cmd)) => crate::commands::new::new(context, cmd, &mut output),
        Some(SubCommand::Set(cmd)) => crate::commands::set::set(context, cmd, &mut output),
        Some(SubCommand::Shell(cmd)) => crate::commands::shell::run(context, cmd, &mut output),
        Some(SubCommand::Edit) => Shell::new(context).edit(&mut output),
        Some(SubCommand::Show) => Shell::new(context).show(&mut output),
        Some(SubCommand::Clear) => Shell::new(context).clear(&mut output),
        Some(SubCommand::As(cmd)) => crate::commands::r#as::r#as(context, cmd, &mut output),
        None => Shell::new(context).execute(&mut output, None),
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
