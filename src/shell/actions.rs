use crate::context::Context;
use crate::option::Options;
use crate::output::Output;
use crate::saucefile::Saucefile;
use crate::shell::utilities::get_binary;
use crate::shell::Shell;

pub fn edit(shell: &dyn Shell, context: Context, output: &mut Output) {
    let path = &context.sauce_path.to_string_lossy();
    let result = shell.edit(path);
    output.push_result(result);
}

pub fn init(shell: &dyn Shell, output: &mut Output, options: &Options) {
    let binary = get_binary();
    let result = shell.init(&binary, options.settings.autoload_hook.unwrap_or(false));
    output.push_result(result);
}

pub fn clear(shell: &dyn Shell, mut saucefile: Saucefile, output: &mut Output, options: &Options) {
    output
        .with_result(render_vars(&mut saucefile, options, |k, _| {
            shell.unset_var(k)
        }))
        .with_result(render_aliases(&mut saucefile, options, |k, _| {
            shell.unset_alias(k)
        }))
        .with_message(render_functions(&mut saucefile, options, |k, _| {
            shell.unset_function(k)
        }))
        .with_message("Cleared your sauce");
}

pub fn show(shell: &dyn Shell, mut saucefile: Saucefile, output: &mut Output, options: &Options) {
    output
        .with_message(render_vars(&mut saucefile, options, |k, v| {
            shell.set_var(k, v)
        }))
        .with_message(render_aliases(&mut saucefile, options, |k, v| {
            shell.set_alias(k, v)
        }))
        .with_message(render_functions(&mut saucefile, options, |k, v| {
            shell.set_function(k, v)
        }));
}

pub fn execute(
    shell: &dyn Shell,
    mut saucefile: Saucefile,
    output: &mut Output,
    options: &Options,
    autoload_flag: bool,
) {
    // The `autoload_flag` indicates that the "context" of the execution is happening during
    // an autoload, i.e. `cd`. It's the precondition for whether we need to check the settings to
    // see whether we **actually** should perform the autoload, or exit early.
    if autoload_flag
        && !saucefile
            .settings()
            .resolve_precedence(&options.settings)
            .autoload
    {
        return;
    }
    output
        .with_result(render_vars(&mut saucefile, options, |k, v| {
            shell.set_var(k, v)
        }))
        .with_result(render_aliases(&mut saucefile, options, |k, v| {
            shell.set_alias(k, v)
        }))
        .with_result(render_functions(&mut saucefile, options, |k, v| {
            shell.set_function(k, v)
        }))
        .with_message(format!("Sourced {}", saucefile.path.to_string_lossy()));
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
            let mut output = Output::default();
            let context = Context {
                sauce_path: Path::new("foo/bar").into(),
                ..Context::default()
            };
            edit(&shell, context, &mut output);
            assert_eq!(output.result(), "edit foo/bar\n");
            assert_eq!(output.message(), "\n");
        }
    }

    mod init {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_defaults() {
            let shell = TestShell {};
            let mut output = Output::default();
            let options = Options::default();

            init(&shell, &mut output, &options);

            assert_eq!(output.result(), "sauce\n");
            assert_eq!(output.message(), "\n");
        }

        #[test]
        fn it_emits_autoload() {
            let shell = TestShell {};
            let mut output = Output::default();
            let mut options = Options::default();

            options.settings.autoload_hook = Some(true);

            init(&shell, &mut output, &options);

            assert_eq!(output.result(), "sauce --autoload\n");
            assert_eq!(output.message(), "\n");
        }
    }

    mod clear {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_clears() {
            let shell = TestShell {};
            let mut output = Output::default();
            let options = Options::default();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            clear(&shell, saucefile, &mut output, &options);

            assert_eq!(
                output.result(),
                "unset var;\nunset alias;\nunset function;\n\n\n"
            );
            assert_eq!(output.message(), "\nCleared your sauce\n");
        }
    }

    mod show {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_shows() {
            let shell = TestShell {};
            let mut output = Output::default();
            let options = Options::default();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            show(&shell, saucefile, &mut output, &options);

            assert_eq!(output.result(), "\n");
            assert_eq!(
                output.message(),
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
            let mut output = Output::default();
            let options = Options::default();
            let mut saucefile = Saucefile::default();
            saucefile.set_var("var", "varvalue");
            saucefile.set_var("alias", "aliasvalue");
            saucefile.set_var("function", "functionvalue");

            execute(&shell, saucefile, &mut output, &options, false);

            assert_eq!(
                output.result(),
                "export var=varvalue;\nexport alias=aliasvalue;\nexport function=functionvalue;\n\n\n\n"
            );
            assert_eq!(output.message(), "Sourced \n");
        }

        #[test]
        fn it_doesnt_execute_with_autoload_flag_and_its_disabled() {
            let shell = TestShell {};
            let mut output = Output::default();
            let options = Options::default();
            let saucefile = Saucefile::default();

            execute(&shell, saucefile, &mut output, &options, true);

            assert_eq!(output.result(), "\n");
            assert_eq!(output.message(), "\n");
        }
    }
}
