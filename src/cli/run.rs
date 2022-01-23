use anyhow::Result;
use etcetera::base_strategy::{BaseStrategy, Xdg};

use super::shape::{CliOptions, KeyValuePair, SetKinds, ShellKinds, ShowKinds, SubCommand};
use crate::cli::utilities::get_input;
use crate::filter::{parse_match_option, FilterOptions};
use crate::output::Output;
use crate::shell::{self, Shell};
use crate::target::Target;
use crate::Context;

pub fn run() -> Result<()> {
    let opts: CliOptions = CliOptions::parse();

    let strategy = Xdg::new()?;
    let config_dir = strategy.config_dir().join("sauce");

    let out = Box::new(std::io::stdout());
    let err = Box::new(std::io::stderr());

    let mut output = Output::new(out, err)
        .quiet(opts.quiet)
        .verbose(opts.verbose)
        .color(shell::should_be_colored(opts.color))
        .only_show(opts.show);

    let filter_options = FilterOptions {
        globs: &parse_match_option(opts.glob.as_deref()),
        filters: &parse_match_option(opts.filter.as_deref()),
        as_: opts.r#as.as_deref(),
        filter_exclusions: &[],
    };

    let mut context = Context::new(
        config_dir,
        filter_options,
        opts.path.as_deref(),
        opts.file.as_deref(),
    )?;

    let shell_kind = &*shell::detect(opts.shell);

    match_subcommmand(
        &mut context,
        &mut output,
        shell_kind,
        opts.subcmd,
        opts.autoload,
    );

    output.flush()?;

    if let Some(code) = output.error_code() {
        std::process::exit(code);
    }

    Ok(())
}

pub fn match_subcommmand(
    context: &mut Context,
    output: &mut Output,
    shell_kind: &dyn Shell,
    subcmd: Option<SubCommand>,
    autoload: bool,
) {
    match subcmd {
        Some(SubCommand::Shell(cmd)) => {
            match cmd.kind {
                ShellKinds::Init => context.init_shell(shell_kind, output),
                ShellKinds::Exec(command) => {
                    context.execute_shell_command(shell_kind, &*command.command, output)
                }
            };
        }
        Some(SubCommand::Config(cmd)) => {
            context.set_config(&cmd.values, cmd.global, output);
        }
        Some(SubCommand::Move(cmd)) => context.move_saucefile(output, &cmd.destination, cmd.copy),
        Some(SubCommand::New) => context.create_saucefile(output),
        Some(SubCommand::Set(cmd)) => match &cmd.kind {
            SetKinds::Env(env) => context.set_var(&get_input(&env.values), output),
            SetKinds::Alias(alias) => context.set_alias(&get_input(&alias.values), output),
            SetKinds::Function(KeyValuePair { key, value }) => {
                context.set_function(key, value, output)
            }
        },
        Some(SubCommand::Edit) => context.edit_saucefile(shell_kind, output),
        Some(SubCommand::Show(show)) => match show.kind {
            ShowKinds::Env => context.show(Target::EnvVar, output),
            ShowKinds::Function => context.show(Target::Function, output),
            ShowKinds::Alias => context.show(Target::Alias, output),
        },
        Some(SubCommand::Clear) => context.clear(shell_kind, output),
        None => context.execute(shell_kind, autoload, output),
    };
}
