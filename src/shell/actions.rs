use crate::context::Context;
use crate::option::GlobalOptions;
use crate::output::Output;
use crate::saucefile::Saucefile;
use crate::shell::utilities::get_binary;
use crate::shell::Shell;

pub fn edit(shell: &dyn Shell, context: Context, output: &mut Output) {
    let path = &context.sauce_path.to_string_lossy();
    let result = shell.edit(path);
    output.push_result(result);
}

pub fn init(shell: &dyn Shell, output: &mut Output) {
    let binary = get_binary();
    let result = shell.init(&binary);
    output.push_result(result);
}

pub fn clear(shell: &dyn Shell, context: Context, output: &mut Output, options: &GlobalOptions) {
    output
        .with_result(render_vars(&context, options, |k, _| shell.unset_var(k)))
        .with_result(render_aliases(&context, options, |k, _| {
            shell.unset_alias(k)
        }))
        .with_message(render_functions(&context, options, |k, _| {
            shell.unset_function(k)
        }))
        .with_message("Cleared your sauce");
}

pub fn show(shell: &dyn Shell, context: Context, output: &mut Output, options: &GlobalOptions) {
    output
        .with_message(render_vars(&context, options, |k, v| shell.set_var(k, v)))
        .with_message(render_aliases(&context, options, |k, v| {
            shell.set_alias(k, v)
        }))
        .with_message(render_functions(&context, options, |k, v| {
            shell.set_function(k, v)
        }));
}

pub fn execute(shell: &dyn Shell, context: Context, output: &mut Output, options: &GlobalOptions) {
    output
        .with_result(render_vars(&context, options, |k, v| shell.set_var(k, v)))
        .with_result(render_aliases(&context, options, |k, v| {
            shell.set_alias(k, v)
        }))
        .with_result(render_functions(&context, options, |k, v| {
            shell.set_function(k, v)
        }))
        .with_message(format!("Sourced {}", context.sauce_path.to_string_lossy()));
}

fn render_vars<F>(context: &Context, options: &GlobalOptions, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    Saucefile::read(context)
        .vars(options)
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|v| format!("{};\n", v))
        .collect()
}

fn render_aliases<F>(context: &Context, options: &GlobalOptions, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    Saucefile::read(context)
        .aliases(options)
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|v| format!("{};\n", v))
        .collect()
}

fn render_functions<F>(context: &Context, options: &GlobalOptions, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    Saucefile::read(context)
        .functions(options)
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|v| format!("{};\n", v))
        .collect()
}
