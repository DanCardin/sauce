use crate::{
    colors::{BLUE, RED, YELLOW},
    saucefile::Saucefile,
    shell::{utilities::get_binary, Shell},
    target::Target,
    Context,
};

pub fn edit(context: &mut Context, shell: &dyn Shell) {
    let path = &context.sauce_path.to_string_lossy();
    context
        .output
        .notify(&[BLUE.paint("Opening "), YELLOW.paint(path.as_ref())]);

    if let Some(result) = shell.edit(std::env::var_os("EDITOR"), path) {
        context.output.output(result);
    } else {
        context
            .output
            .notify(&[RED.paint("set $EDITOR to enable this command")]);
    }
}

pub fn init(context: &mut Context, shell: &dyn Shell) {
    let binary = get_binary();
    let result = shell.init(&binary, context.settings.autoload_hook.unwrap_or(false));
    context.output.output(result);
}

pub fn execute_shell_command(context: &mut Context, shell: &dyn Shell, command: &str) {
    let result = subprocess::Exec::cmd(shell.name())
        .arg("-i")
        .arg("-c")
        .arg(format!("{}; {}", clap::crate_name!(), command))
        .stdout(subprocess::Redirection::Merge)
        .join();

    if let Err(error) = result {
        context
            .output
            .notify(&[RED.bold().paint(error.to_string())]);
    }
}

pub fn clear(context: &mut Context, shell: &dyn Shell, mut saucefile: Saucefile) {
    let options = &context.options;
    let output = &mut context.output;
    output.output(render_items(saucefile.vars(options), |k, _| {
        shell.unset_var(k)
    }));
    output.output(render_items(saucefile.aliases(options), |k, _| {
        shell.unset_alias(k)
    }));
    output.output(render_items(saucefile.functions(options), |k, _| {
        shell.unset_function(k)
    }));
    output.notify(&[BLUE.bold().paint("Cleared your sauce")]);
}

pub fn show(context: &mut Context, target: Target, mut saucefile: Saucefile) {
    let header = match target {
        Target::EnvVar => &["Variable", "Value"],
        Target::Alias => &["Alias", "Value"],
        Target::Function => &["Function", "Body"],
    };

    let options = &context.options;
    let pairs = match target {
        Target::EnvVar => saucefile.vars(options),
        Target::Alias => saucefile.aliases(options),
        Target::Function => saucefile.functions(options),
    };
    let preset = match target {
        Target::EnvVar => None,
        Target::Alias => None,
        Target::Function => Some("││──╞═╪╡│ │││┬┴┌┐└┘"),
    };

    let cells = pairs
        .iter()
        .map(|(k, v)| vec![<&str>::clone(k), v])
        .collect::<Vec<_>>();
    let table = context.output.format_table(header, cells, preset);

    context.output.notify_str(&table);
}

pub fn execute(
    context: &mut Context,
    shell: &dyn Shell,
    mut saucefile: Saucefile,
    autoload_flag: bool,
) {
    // The `autoload_flag` indicates that the "context" of the execution is happening during
    // an autoload, i.e. `cd`. It's the precondition for whether we need to check the settings to
    // see whether we **actually** should perform the autoload, or exit early.
    if autoload_flag
        && !saucefile
            .settings()
            .resolve_precedence(&context.settings)
            .autoload
    {
        return;
    }
    let options = &context.options;
    let output = &mut context.output;

    output.output(render_items(saucefile.vars(options), |k, v| {
        shell.set_var(k, v)
    }));
    output.output(render_items(saucefile.aliases(options), |k, v| {
        shell.set_alias(k, v)
    }));
    output.output(render_items(saucefile.functions(options), |k, v| {
        shell.set_function(k, v)
    }));

    output.notify(&[
        BLUE.bold().paint("Sourced "),
        YELLOW.paint(saucefile.path.to_string_lossy()),
    ]);
}

fn render_items<F>(items: Vec<(&str, String)>, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    items
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|mut v| {
            v += ";\n";
            v
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{setup, TestShell};
    use indoc::indoc;
    use std::path::Path;

    mod edit {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_respects_editor_env_var_zsh() {
            std::env::set_var("EDITOR", "edit");

            let (out, err, mut context) = setup();
            context.sauce_path = Path::new("foo/bar").into();

            let shell = TestShell {};
            edit(&mut context, &shell);

            assert_eq!(out.value(), "edit 'foo/bar'\n");
            assert_eq!(err.value(), "Opening foo/bar\n");
        }
    }

    mod init {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_defaults() {
            let (out, err, mut context) = setup();
            let shell = TestShell {};
            init(&mut context, &shell);

            assert_eq!(out.value(), "sauce\n");
            assert_eq!(err.value(), "");
        }

        #[test]
        fn it_emits_autoload() {
            let (out, err, mut context) = setup();
            let shell = TestShell {};

            context.settings.autoload_hook = Some(true);

            init(&mut context, &shell);

            assert_eq!(out.value(), "sauce --autoload\n");
            assert_eq!(err.value(), "");
        }
    }

    mod clear {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_clears() {
            let shell = TestShell {};
            let (out, err, mut context) = setup();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            clear(&mut context, &shell, saucefile);

            assert_eq!(out.value(), "unset var;\nunset alias;\nunset function;\n\n");
            assert_eq!(err.value(), "Cleared your sauce\n");
        }
    }

    mod show {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_shows_env_vars() {
            let (out, err, mut context) = setup();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");

            show(&mut context, Target::EnvVar, saucefile);

            assert_eq!(out.value(), "");
            assert_eq!(
                err.value(),
                indoc!(
                    "
                    ┌──────────┬──────────┐
                    │ Variable │ Value    │
                    ╞══════════╪══════════╡
                    │ var      │ varvalue │
                    └──────────┴──────────┘
                    "
                )
            );
        }

        #[test]
        fn it_shows_aliases() {
            let (out, err, mut context) = setup();
            let mut saucefile = Saucefile::default();
            saucefile.set_alias("alias", "aliasvalue");

            show(&mut context, Target::Alias, saucefile);

            assert_eq!(out.value(), "");
            assert_eq!(
                err.value(),
                indoc!(
                    "
                    ┌───────┬────────────┐
                    │ Alias │ Value      │
                    ╞═══════╪════════════╡
                    │ alias │ aliasvalue │
                    └───────┴────────────┘
                    "
                )
            );
        }

        #[test]
        fn it_shows_functions() {
            let (out, err, mut context) = setup();
            let mut saucefile = Saucefile::default();
            saucefile.set_function("function", "git add\ngit commit");

            show(&mut context, Target::Function, saucefile);

            assert_eq!(out.value(), "");
            assert_eq!(
                err.value(),
                indoc!(
                    "
                    ┌──────────┬────────────┐
                    │ Function │ Body       │
                    ╞══════════╪════════════╡
                    │ function │ git add    │
                    │          │ git commit │
                    └──────────┴────────────┘
                    "
                )
            );
        }
    }

    mod execute {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_executes() {
            let shell = TestShell {};
            let (out, err, mut context) = setup();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            execute(&mut context, &shell, saucefile, false);

            assert_eq!(
                out.value(),
                "export var=varvalue;\nexport alias=aliasvalue;\nexport function=functionvalue;\n\n"
            );
            assert_eq!(err.value(), "Sourced \n");
        }

        #[test]
        fn it_doesnt_execute_with_autoload_flag_and_its_disabled() {
            let shell = TestShell {};
            let (out, err, mut context) = setup();
            let saucefile = Saucefile::default();

            execute(&mut context, &shell, saucefile, true);

            assert_eq!(out.value(), "");
            assert_eq!(err.value(), "");
        }
    }
}
