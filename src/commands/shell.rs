use clap::Clap;

use crate::output::Output;
use crate::shell::{actions, Shell};

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

pub fn run(shell_kind: &dyn Shell, cmd: ShellCommand, output: &mut Output) {
    match cmd.kind {
        Some(ShellKinds::Init) => actions::init(shell_kind, output),
        None => return, // shell.create_subshell(output),
    }
}
