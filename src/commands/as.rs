use crate::context::Context;
use crate::output::Output;
use crate::shell::Shell;
use clap::Clap;

/// Sauces "as" a given tag.
#[derive(Clap, Debug)]
pub struct AsCommand {
    /// Any value, typically just a literal, can be changed to a table. The
    /// keys of this table act as a tag.
    ///
    /// # Examples
    ///
    /// ```toml
    /// [vars]
    /// DATABASE_USER = {default = "postgres", dev = "postgres", prod = "abcd"}
    /// ```
    r#as: String,
}

pub fn r#as(context: Context, cmd: AsCommand, output: &mut Output) {
    output.push_message(format!("Saucing as {}", cmd.r#as));
    Shell::new(context).execute(output, Some(&cmd.r#as));
}
