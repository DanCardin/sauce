use ansi_term::Style;

use crate::{
    colors::{BLUE, YELLOW},
    option::Options,
    saucefile::Saucefile,
    shell::{utilities::get_binary, Shell},
    Context,
};

pub fn edit(context: &mut Context, shell: &dyn Shell) {
    let path = &context.sauce_path.to_string_lossy();
    let result = shell.edit(path);
    context.output.output(result);
}

pub fn init(context: &mut Context, shell: &dyn Shell) {
    let binary = get_binary();
    let result = shell.init(&binary, context.settings.autoload_hook.unwrap_or(false));
    context.output.output(result);
}

pub fn clear(context: &mut Context, shell: &dyn Shell, mut saucefile: Saucefile) {
    let options = &context.options;
    let output = &mut context.output;
    output.output(render_vars(&mut saucefile, options, |k, _| {
        shell.unset_var(k)
    }));
    output.output(render_aliases(&mut saucefile, options, |k, _| {
        shell.unset_alias(k)
    }));
    output.output(render_functions(&mut saucefile, options, |k, _| {
        shell.unset_function(k)
    }));
    output.notify(&[BLUE.bold().paint("Cleared your sauce")]);
}

pub fn show(context: &mut Context, shell: &dyn Shell, mut saucefile: Saucefile) {
    let options = &context.options;
    let output = &mut context.output;

    let vars = render_vars(&mut saucefile, options, |k, v| shell.set_var(k, v));
    let aliases = render_aliases(&mut saucefile, options, |k, v| shell.set_alias(k, v));
    let functions = render_functions(&mut saucefile, options, |k, v| shell.set_function(k, v));
    output.notify(&[Style::default().paint(vars)]);
    output.notify(&[Style::default().paint(aliases)]);
    output.notify(&[Style::default().paint(functions)]);
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

    output.output(render_vars(&mut saucefile, options, |k, v| {
        shell.set_var(k, v)
    }));
    output.output(render_aliases(&mut saucefile, options, |k, v| {
        shell.set_alias(k, v)
    }));
    output.output(render_functions(&mut saucefile, options, |k, v| {
        shell.set_function(k, v)
    }));

    output.notify(&[
        BLUE.bold().paint("Sourced "),
        YELLOW.paint(saucefile.path.to_string_lossy()),
    ]);
}

fn render_vars<F>(saucefile: &mut Saucefile, options: &Options, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    saucefile
        .vars(options)
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|v| format!("{};\n", v))
        .collect()
}

fn render_aliases<F>(saucefile: &mut Saucefile, options: &Options, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    saucefile
        .aliases(options)
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|v| format!("{};\n", v))
        .collect()
}

fn render_functions<F>(saucefile: &mut Saucefile, options: &Options, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    saucefile
        .functions(options)
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|v| format!("{};\n", v))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{setup, TestShell};
    use std::path::Path;

    mod edit {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_respects_editor_env_var_zsh() {
            let (out, err, mut context) = setup();
            context.sauce_path = Path::new("foo/bar").into();

            let shell = TestShell {};
            edit(&mut context, &shell);
            assert_eq!(out.value(), "edit foo/bar\n");
            assert_eq!(err.value(), "");
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
        fn it_shows() {
            let shell = TestShell {};
            let (out, err, mut context) = setup();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            show(&mut context, &shell, saucefile);

            assert_eq!(out.value(), "");
            assert_eq!(
                err.value(),
                "export var=varvalue;\nexport alias=aliasvalue;\nexport function=functionvalue;\n\n"
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
