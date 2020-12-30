use crate::context::Context;
use crate::output::Output;
use crate::saucefile::Saucefile;
use anyhow::Result;
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
}

/// Environment variable type
#[derive(Clap, Debug)]
struct SetVarKind {
    values: Vec<String>,
}

pub fn set(context: Context, cmd: SetCommand) -> Result<Output> {
    let saucefile = Saucefile::read(&context);
    match cmd.kind {
        SetKinds::Var(var) => set_var(&context, saucefile, var),
    }
}

fn set_var(context: &Context, mut saucefile: Saucefile, opts: SetVarKind) -> Result<Output> {
    let mut output = Output::default();

    for values in opts.values.iter() {
        let parts: Vec<&str> = values.splitn(2, '=').collect();
        let var = parts[0];
        let value = if parts.len() > 1 { parts[1] } else { "" };
        saucefile.set_var(var.to_string(), value.to_string());
        output.push_message(format!("Set '{}' to {}", var, value));
    }
    saucefile.write(&context)?;
    Ok(output)
}
