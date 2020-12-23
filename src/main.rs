mod context;
mod saucefile;

mod commands {
    pub mod new {
        use crate::context::Context;
        use anyhow::Result;
        use clap::Clap;

        /// Creates a sauce file
        #[derive(Clap, Debug)]
        pub struct NewCommand {
            /// Print debug info
            #[clap(short)]
            debug: bool,
        }

        pub fn new(context: Context, _cmd: NewCommand) -> Result<()> {
            let parent = context.sauce_path.parent().unwrap();
            std::fs::create_dir_all(parent)?;

            if context.sauce_path.is_file() {
                println!(
                    "File already exists at {}",
                    context.sauce_path.to_string_lossy()
                );
                return Ok(());
            }
            std::fs::File::create(context.sauce_path)?;
            println!("Created");
            Ok(())
        }
    }

    pub mod add {
        use crate::context::Context;
        use crate::saucefile::Saucefile;
        use anyhow::Result;
        use clap::Clap;

        /// Adds to the sauce file
        #[derive(Clap, Debug)]
        pub struct AddCommand {
            /// the kind of thing to add
            #[clap(subcommand)]
            kind: AddKinds,
        }

        #[derive(Clap, Debug)]
        enum AddKinds {
            Var(AddVarKind),
        }

        /// Environment variable type
        #[derive(Clap, Debug)]
        struct AddVarKind {
            additions: Vec<String>,
        }

        pub fn add(context: Context, cmd: AddCommand) -> Result<()> {
            let mut saucefile = Saucefile::read(&context);
            println!("add {:?}", saucefile);

            match cmd.kind {
                AddKinds::Var(var) => add_var(&mut saucefile, var),
            }?;

            saucefile.write(&context)
        }

        fn add_var(saucefile: &mut Saucefile, opts: AddVarKind) -> Result<()> {
            println!("add {:?}", opts);

            for addition in opts.additions.iter() {
                let parts: Vec<&str> = addition.splitn(2, '=').collect();
                let var = parts[0];
                let value = if parts.len() > 1 { parts[1] } else { "" };
                saucefile.add_var(var.to_string(), value.to_string());
            }
            println!("add {:?}", saucefile);
            Ok(())
        }
    }

    pub mod shell {
        use clap::Clap;

        use crate::context::Context;

        use anyhow::Result;

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

        pub fn run(context: Context, cmd: ShellCommand) -> Result<()> {
            match cmd.kind {
                Some(ShellKinds::Init) => crate::shell::init(context),
                None => Ok(()), //create_subshell(),
            }
        }
    }
}

mod shell {
    use std::io::Write;

    use crate::context::Context;
    use crate::saucefile::Saucefile;
    use anyhow::Result;

    pub fn clear(context: Context) -> Result<()> {
        let saucefile = Saucefile::read(&context);

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        for var in saucefile.vars.keys() {
            let statement = format!("unset {}\n", var);
            handle.write_all(statement.as_ref())?;
        }

        Ok(())
    }

    pub fn execute(context: Context) -> Result<()> {
        let saucefile = Saucefile::read(&context);

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        for (var, value) in saucefile.vars.iter() {
            let statement = format!("export {}={}\n", var, value);
            handle.write_all(statement.as_ref())?;
        }

        Ok(())
    }

    pub fn init(_context: Context) -> Result<()> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        let statement = format!(
            r#"function sauce() {{ $(command {} "$&") }}"#,
            clap::crate_name!()
        );
        handle.write_all(statement.as_ref())?;
        Ok(())
    }
}

use crate::commands::add::AddCommand;
use crate::commands::new::NewCommand;
use crate::commands::shell::ShellCommand;
use crate::context::Context;
use anyhow::Result;

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
    Add(AddCommand),
    Shell(ShellCommand),
    Clear,
}

fn main() -> Result<()> {
    let opts: Options = Options::parse();
    // println!("{:?}!", opts);
    let context = Context::new()?;

    match opts.subcmd {
        Some(SubCommand::New(cmd)) => crate::commands::new::new(context, cmd),
        Some(SubCommand::Add(cmd)) => crate::commands::add::add(context, cmd),
        Some(SubCommand::Shell(cmd)) => crate::commands::shell::run(context, cmd),
        Some(SubCommand::Clear) => crate::shell::clear(context),
        None => crate::shell::execute(context),
    }?;
    Ok(())
}
