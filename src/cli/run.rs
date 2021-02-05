use crate::cli::utilities::get_input;
use crate::option::Options;
use crate::output::Output;
use crate::settings::Settings;
use crate::shell::{self, Shell};
use crate::Context;
use anyhow::Result;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};

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

    let out = Box::new(std::io::stdout());
    let err = Box::new(std::io::stderr());

    let color_enabled = shell::should_be_colored(opts.color);
    let mut output = Output::new(out, err, color_enabled, opts.quiet, opts.verbose);

    let settings = Settings::load(&config_dir, &mut output)?;
    let options = Options::new(
        opts.glob.as_deref(),
        opts.filter.as_deref(),
        opts.r#as.as_deref(),
        opts.path.as_deref(),
    );

    let mut context = Context::new(data_dir, settings, options, output)?;

    let shell_kind = &*shell::detect(opts.shell);

    match_subcommmand(&mut context, shell_kind, opts.subcmd, opts.autoload);

    context.flush()?;

    if let Some(code) = context.output.error_code() {
        std::process::exit(code);
    }

    Ok(())
}

pub fn match_subcommmand(
    context: &mut Context,
    shell_kind: &dyn Shell,
    subcmd: Option<SubCommand>,
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
            SetKinds::Env(env) => context.set_var(&get_input(&env.values)),
            SetKinds::Alias(alias) => context.set_alias(&get_input(&alias.values)),
            SetKinds::Function(KeyValuePair { key, value }) => context.set_function(key, value),
        },
        Some(SubCommand::Edit) => context.edit_saucefile(shell_kind),
        Some(SubCommand::Show) => context.show(shell_kind),
        Some(SubCommand::Clear) => context.clear(shell_kind),
        None => context.execute(shell_kind, autoload),
    };
}
