use clap::Clap;

use crate::context::Context;
use crate::output::Output;
use crate::shell::Shell;

/// Adds to the sauce file
#[derive(Clap, Debug)]
pub struct ShellCommand {
    /// the kind of thing to add
    #[clap(subcommand)]
    pub kind: Option<ShellKinds>,
}

#[derive(Clap, Debug)]
pub enum ShellKinds {
    Init,
}

pub fn run(context: Context, cmd: ShellCommand, output: &mut Output) {
    let shell = Shell::new(context);
    match cmd.kind {
        Some(ShellKinds::Init) => shell.init(output),
        None => shell.create_subshell(output),
    }
}
