use crate::context::Context;
use crate::output::Output;
use crate::saucefile::Saucefile;
use clap::Clap;
use std::io::Read;
use std::ops::Deref;

/// Sets to the sauce file
#[derive(Clap, Debug)]
pub struct SetCommand {
    /// the kind of thing to set
    #[clap(subcommand)]
    kind: SetKinds,
}

#[derive(Clap, Debug)]
enum SetKinds {
    Var(SetVarKind),
    Alias(SetVarKind),
}

/// Environment variable type
#[derive(Clap, Debug)]
struct SetVarKind {
    values: Vec<String>,
}

fn get_input(mut values: Vec<String>) -> Vec<String> {
    let in_ = std::io::stdin();
    let mut handle = in_.lock();

    let mut buffer = String::new();
    handle.read_to_string(&mut buffer).unwrap();
    if !buffer.is_empty() {
        if let Some(b) = buffer.strip_suffix("\n") {
            buffer = b.to_string();
        }
        values.push(buffer);
    }

    values
}

pub fn set(context: Context, cmd: SetCommand, output: &mut Output) {
    let saucefile = Saucefile::read(&context);
    match cmd.kind {
        SetKinds::Var(var) => set_var(&context, saucefile, get_input(var.values), output),
        SetKinds::Alias(alias) => set_alias(&context, saucefile, get_input(alias.values), output),
    }
}

fn set_var(context: &Context, mut saucefile: Saucefile, values: Vec<String>, output: &mut Output) {
    for values in values.iter() {
        let parts: Vec<&str> = values.splitn(2, '=').collect();
        let var = parts[0];

        let value = parts.get(1).map(Deref::deref).unwrap_or("");

        saucefile.set_var(var, value);
        output.push_message(format!("Set '{}' to {}", var, value));
    }
    if saucefile.write(&context).is_err() {
        output.push_message("couldn't write the thing")
    }
}

fn set_alias(
    context: &Context,
    mut saucefile: Saucefile,
    values: Vec<String>,
    output: &mut Output,
) {
    for values in values.iter() {
        let parts: Vec<&str> = values.splitn(2, '=').collect();
        let var = parts[0];
        let value = if parts.len() > 1 { parts[1] } else { "" };
        saucefile.set_alias(var.to_string(), value.to_string());
        output.push_message(format!("Set '{}' to {}", var, value));
    }
    if saucefile.write(&context).is_err() {
        output.push_message("couldn't writ the thing")
    }
}
