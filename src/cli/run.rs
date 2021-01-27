use crate::cli::utilities::get_input;
use crate::option::Options;
use crate::output::Output;
use crate::settings::Settings;
use crate::shell::{self, Shell};
use crate::Context;
use anyhow::Result;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use std::io::Write;

use super::shape::{CliOptions, KeyValuePair, SetKinds, ShellKinds, SubCommand};

pub fn run() -> Result<()> {
    let opts: CliOptions = CliOptions::parse();

    let strat_args = AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "dancardin".to_string(),
        app_name: "sauce".to_string(),
    };
    let strategy = Xdg::new(strat_args)?;
    let data_dir = strategy.data_dir();
    let config_dir = strategy.config_dir();

    let settings = Settings::load(&config_dir)?;
    let options = Options::new(
        opts.glob.as_deref(),
        opts.filter.as_deref(),
        opts.r#as.as_deref(),
        opts.path.as_deref(),
    );

    let context = Context::new(data_dir, settings, options)?;

    let shell_kind = &*shell::detect(opts.shell);

    let output = match_subcommmand(context, shell_kind, &opts.subcmd, opts.autoload);

    let out = std::io::stdout();
    let mut handle = out.lock();
    handle.write_all(output.result().as_ref())?;
    handle.flush()?;

    let out = std::io::stderr();
    let mut handle = out.lock();
    handle.write_all(output.message().as_ref())?;
    handle.flush()?;

    Ok(())
}

pub fn match_subcommmand(
    context: Context,
    shell_kind: &dyn Shell,
    subcmd: &Option<SubCommand>,
    autoload: bool,
) -> Output {
    let mut output = Output::default();

    match subcmd {
        Some(SubCommand::Shell(cmd)) => {
            match cmd.kind {
                ShellKinds::Init => context.init_shell(shell_kind, &mut output),
            };
        }
        Some(SubCommand::Config(cmd)) => {
            context.set_config(&cmd.values, cmd.global, &mut output);
        }
        Some(SubCommand::New) => context.create_saucefile(&mut output),
        Some(SubCommand::Set(cmd)) => match &cmd.kind {
            SetKinds::Var(var) => context.set_var(&get_input(&var.values), &mut output),
            SetKinds::Alias(alias) => context.set_alias(&get_input(&alias.values), &mut output),
            SetKinds::Function(KeyValuePair { key, value }) => {
                context.set_function(key, value, &mut output)
            }
        },
        Some(SubCommand::Edit) => context.edit_saucefile(shell_kind, &mut output),
        Some(SubCommand::Show) => context.show(shell_kind, &mut output),
        Some(SubCommand::Clear) => context.clear(shell_kind, &mut output),
        None => context.execute(shell_kind, &mut output, autoload),
    };

    output
}
