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
) {
    if !saucefile
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
