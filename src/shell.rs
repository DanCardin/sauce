use crate::context::Context;
use crate::option::GlobalOptions;
use crate::output::Output;
use crate::saucefile::Saucefile;

fn escape(value: &str) -> String {
    snailquote::escape(&value.to_string()).replace("\\n", "\n")
}

pub struct Shell {
    context: Context,
}

impl<'a> Shell {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub fn edit(&self, output: &mut Output) {
        output.push_result(format!(
            "\"$EDITOR\" '{}'",
            self.context.sauce_path.to_string_lossy()
        ))
    }

    pub fn init(&self, output: &mut Output) {
        let prefix = if cfg!(debug_assertions) {
            "./target/debug/"
        } else {
            ""
        };
        let statement = format!(
            r#"
            sauce() {{
                eval "$(command {}{} "$@")"
            }}
            "#,
            prefix,
            clap::crate_name!(),
        );
        output.push_result(statement);
    }

    pub fn clear(&self, output: &mut Output, options: GlobalOptions) {
        output
            .with_result(self.render_vars(|var, _| format!("unset {}", var), options.as_))
            .with_result(
                self.render_aliases(|var, _| format!("unalias {} 2>/dev/null", var), options.as_),
            )
            .with_message("Cleared your sauce");
    }

    pub fn show(&self, output: &mut Output, options: GlobalOptions) {
        output
            .with_message(self.render_vars(
                |var, value| format!("export {}={}", var, escape(&value)),
                options.as_,
            ))
            .with_message(self.render_aliases(
                |var, value| format!("alias {}={}", var, escape(&value)),
                options.as_,
            ))
            .with_message(self.render_functions(options.as_));
    }

    pub fn execute(&self, output: &mut Output, options: GlobalOptions) {
        output
            .with_result(self.render_vars(
                |var, value| format!("export {}={}", var, escape(&value)),
                options.as_,
            ))
            .with_result(self.render_aliases(
                |var, value| format!("alias {}={}", var, escape(&value)),
                options.as_,
            ))
            .with_result(self.render_functions(options.as_))
            .with_message(format!(
                "Sourced {}",
                self.context.sauce_path.to_string_lossy()
            ));
    }

    pub fn create_subshell(&self, _output: &mut Output) {}

    fn render_vars<F>(&self, mut format_row: F, tag: Option<&str>) -> String
    where
        F: FnMut(&String, &String) -> String,
    {
        Saucefile::read(&self.context)
            .vars(tag)
            .iter()
            .map(|(k, v)| format_row(k, v))
            .map(|v| format!("{};\n", v))
            .collect()
    }

    fn render_aliases<F>(&self, mut format_row: F, tag: Option<&str>) -> String
    where
        F: FnMut(&String, &String) -> String,
    {
        Saucefile::read(&self.context)
            .aliases(tag)
            .iter()
            .map(|(k, v)| format_row(k, v))
            .map(|v| format!("{};\n", v))
            .collect()
    }

    fn render_functions(&self, tag: Option<&str>) -> String {
        Saucefile::read(&self.context)
            .functions(tag)
            .iter()
            .map(|(k, v)| format!("function {} {{\n  {}\n}};\n", k, v.replace("\n", "\n  ")))
            .collect()
    }
}
