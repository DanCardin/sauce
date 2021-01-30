use ansi_term::ANSIString;

use crate::shell::utilities::get_binary;
use crate::shell::Shell;
use crate::Context;
use crate::{colors::BLUE, option::Options};
use crate::{colors::YELLOW, saucefile::Saucefile};

pub fn edit(context: &mut Context, shell: &dyn Shell) {
    let path = &context.sauce_path.to_string_lossy();
    let result = shell.edit(path);
    context.output.push_result(result);
}

pub fn init(context: &mut Context, shell: &dyn Shell) {
    let binary = get_binary();
    let result = shell.init(&binary, context.settings.autoload_hook.unwrap_or(false));
    context.output.push_result(result);
}

pub fn clear(context: &mut Context, shell: &dyn Shell, mut saucefile: Saucefile) {
    let options = &context.options;
    let output = &mut context.output;
    output.push_result(render_vars(&mut saucefile, options, |k, _| {
        shell.unset_var(k)
    }));
    output.push_result(render_aliases(&mut saucefile, options, |k, _| {
        shell.unset_alias(k)
    }));
    output.push_result(render_functions(&mut saucefile, options, |k, _| {
        shell.unset_function(k)
    }));
    output.push_message(&[BLUE.bold().paint("Cleared your sauce")]);
}

pub fn show(context: &mut Context, shell: &dyn Shell, mut saucefile: Saucefile) {
    let options = &context.options;
    let output = &mut context.output;
    output.push_message(&[ANSIString::from(render_vars(
        &mut saucefile,
        options,
        |k, v| shell.set_var(k, v),
    ))]);
    output.push_message(&[ANSIString::from(render_aliases(
        &mut saucefile,
        options,
        |k, v| shell.set_alias(k, v),
    ))]);
    output.push_message(&[ANSIString::from(render_functions(
        &mut saucefile,
        options,
        |k, v| shell.set_function(k, v),
    ))]);
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

    output.push_result(render_vars(&mut saucefile, options, |k, v| {
        shell.set_var(k, v)
    }));
    output.push_result(render_aliases(&mut saucefile, options, |k, v| {
        shell.set_alias(k, v)
    }));
    output.push_result(render_functions(&mut saucefile, options, |k, v| {
        shell.set_function(k, v)
    }));
    output.push_message(&[
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
    use crate::shell::Shell;

    pub struct TestShell;

    impl Shell for TestShell {
        fn edit(&self, path: &str) -> String {
            format!("edit {}", path)
        }

        fn init(&self, binary: &str, autoload_hook: bool) -> String {
            if autoload_hook {
                format!("{} {}", binary, "--autoload")
            } else {
                format!("{}", binary)
            }
        }

        fn set_var(&self, var: &str, value: &str) -> String {
            format!("export {}={}", var, value)
        }

        fn set_alias(&self, var: &str, value: &str) -> String {
            format!("alias {}={}", var, value)
        }

        fn set_function(&self, var: &str, value: &str) -> String {
            format!("function {}={}", var, value)
        }

        fn unset_var(&self, var: &str) -> String {
            format!("unset {}", var)
        }

        fn unset_alias(&self, var: &str) -> String {
            format!("unalias {}", var)
        }

        fn unset_function(&self, var: &str) -> String {
            format!("unset {}", var)
        }
    }

    mod edit {
        use std::path::Path;

        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_respects_editor_env_var_zsh() {
            let shell = TestShell {};
            let mut context = Context {
                sauce_path: Path::new("foo/bar").into(),
                ..Context::default()
            };
            edit(&mut context, &shell);
            assert_eq!(context.output.result(), "edit foo/bar\n");
            assert_eq!(context.output.message(), "\n");
        }
    }

    mod init {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_defaults() {
            let shell = TestShell {};
            let mut context = Context::default();
            init(&mut context, &shell);

            assert_eq!(context.output.result(), "sauce\n");
            assert_eq!(context.output.message(), "\n");
        }

        #[test]
        fn it_emits_autoload() {
            let shell = TestShell {};
            let mut context = Context::default();

            context.settings.autoload_hook = Some(true);

            init(&mut context, &shell);

            assert_eq!(context.output.result(), "sauce --autoload\n");
            assert_eq!(context.output.message(), "\n");
        }
    }

    mod clear {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_clears() {
            let shell = TestShell {};
            let mut context = Context::default();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            clear(&mut context, &shell, saucefile);

            assert_eq!(
                context.output.result(),
                "unset var;\nunset alias;\nunset function;\n\n\n"
            );
            assert_eq!(context.output.message(), "\nCleared your sauce\n");
        }
    }

    mod show {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_shows() {
            let shell = TestShell {};
            let mut context = Context::default();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            show(&mut context, &shell, saucefile);

            assert_eq!(context.output.result(), "\n");
            assert_eq!(
                context.output.message(),
                "export var=varvalue;\nexport alias=aliasvalue;\nexport function=functionvalue;\n\n\n\n"
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
            let mut context = Context::default();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            execute(&mut context, &shell, saucefile, false);

            assert_eq!(
                context.output.result(),
                "export var=varvalue;\nexport alias=aliasvalue;\nexport function=functionvalue;\n\n\n\n"
            );
            assert_eq!(context.output.message(), "Sourced \n");
        }

        #[test]
        fn it_doesnt_execute_with_autoload_flag_and_its_disabled() {
            let shell = TestShell {};
            let mut context = Context::default();
            let saucefile = Saucefile::default();

            execute(&mut context, &shell, saucefile, true);

            assert_eq!(context.output.result(), "\n");
            assert_eq!(context.output.message(), "\n");
        }
    }
}
