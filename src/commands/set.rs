use crate::context::Context;
use crate::output::Output;
use crate::saucefile::Saucefile;
use clap::Clap;

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

pub fn set(context: Context, cmd: SetCommand, output: &mut Output) {
    let saucefile = Saucefile::read(&context);
    match cmd.kind {
        SetKinds::Var(var) => set_var(&context, saucefile, var, output),
        SetKinds::Alias(alias) => set_alias(&context, saucefile, alias, output),
    }
}

fn set_var(context: &Context, mut saucefile: Saucefile, opts: SetVarKind, output: &mut Output) {
    for values in opts.values.iter() {
        let parts: Vec<&str> = values.splitn(2, '=').collect();
        let var = parts[0];
        let value = if parts.len() > 1 { parts[1] } else { "" };
        saucefile.set_var(var.to_string(), value.to_string());
        output.push_message(format!("Set '{}' to {}", var, value));
    }
    if saucefile.write(&context).is_err() {
        output.push_message("couldn't write the thing")
    }
}

fn set_alias(context: &Context, mut saucefile: Saucefile, opts: SetVarKind, output: &mut Output) {
    for values in opts.values.iter() {
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
