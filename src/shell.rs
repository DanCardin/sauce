use crate::context::Context;
use crate::output::Output;
use crate::saucefile::Saucefile;

pub struct Shell {
    context: Context,
}

impl Shell {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    fn render<F>(&self, mut format_row: F, tag: Option<&str>) -> String
    where
        F: FnMut(&String, &String) -> String,
    {
        Saucefile::read(&self.context)
            .vars(tag)
            .iter()
            .map(|(k, v)| format_row(k, v))
            .map(|v| format!("{}\n", v))
            .collect()
    }

    pub fn clear(&self, output: &mut Output) {
        output
            .with_result(self.render(|var, _| format!("unset {}", var), None))
            .with_message("Cleared your sauce");
    }

    pub fn show(&self, output: &mut Output) {
        output.push_message(self.render(
            |var, value| format!("export {}={}", var, shell_words::quote(&value.to_string())),
            None,
        ))
    }

    pub fn edit(&self, output: &mut Output) {
        output.push_result(format!(
            "\"$EDITOR\" '{}'",
            self.context.sauce_path.to_string_lossy()
        ))
    }

    pub fn execute(&self, output: &mut Output, tag: Option<&str>) {
        output
            .with_result(self.render(|var, value| format!("export {}={}", var, value), tag))
            .push_message(format!(
                "Sourced {}",
                self.context.sauce_path.to_string_lossy()
            ));
    }

    pub fn init(&self, output: &mut Output) {
        let executable = if cfg!(debug_assertions) {
            "./target/debug/sauce"
        } else {
            clap::crate_name!()
        };
        let statement = format!(
            r#"
            sauce() {{
                eval $(command {} $@)
            }}
            "#,
            executable
        );
        output.push_result(statement);
    }

    pub fn create_subshell(&self, _output: &mut Output) {}
}
