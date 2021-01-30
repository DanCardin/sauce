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

    let mut output = Output::default();

    let settings = Settings::load(&config_dir, &mut output)?;
    let options = Options::new(
        opts.glob.as_deref(),
        opts.filter.as_deref(),
        opts.r#as.as_deref(),
        opts.path.as_deref(),
    );

    let mut context = Context::new(data_dir, settings, options, output)?;

    let shell_kind = &*shell::detect(opts.shell);

    match_subcommmand(&mut context, shell_kind, &opts.subcmd, opts.autoload);

    let mut out = std::io::stdout();
    let mut err = std::io::stderr();
    context.write(&mut out, &mut err, shell::should_be_colored(opts.color))?;
    out.flush()?;
    err.flush()?;

    if let Some(code) = context.output.error_code() {
        std::process::exit(code);
    }

    Ok(())
}

pub fn match_subcommmand<'a>(
    context: &'a mut Context<'a>,
    shell_kind: &dyn Shell,
    subcmd: &'a Option<SubCommand>,
    autoload: bool,
) {
    match subcmd {
        Some(SubCommand::Shell(cmd)) => {
            match cmd.kind {
                ShellKinds::Init => context.init_shell(shell_kind),
            };
        }
        Some(SubCommand::Config(cmd)) => {
            context.set_config(&cmd.values, cmd.global);
        }
        Some(SubCommand::New) => context.create_saucefile(),
        Some(SubCommand::Set(cmd)) => match &cmd.kind {
            SetKinds::Var(var) => context.set_var(&get_input(&var.values)),
            SetKinds::Alias(alias) => context.set_alias(&get_input(&alias.values)),
            SetKinds::Function(KeyValuePair { key, value }) => context.set_function(key, value),
        },
        Some(SubCommand::Edit) => context.edit_saucefile(shell_kind),
        Some(SubCommand::Show) => context.show(shell_kind),
        Some(SubCommand::Clear) => context.clear(shell_kind),
        None => context.execute(shell_kind, autoload),
    };
}
