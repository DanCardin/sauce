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

    fn render<F>(&self, mut format_row: F) -> String
    where
        F: FnMut(&String, &String) -> String,
    {
        Saucefile::read(&self.context)
            .vars()
            .iter()
            .map(|(k, v)| format_row(k, v))
            .map(|v| format!("{}\n", v))
            .collect()
    }

    pub fn clear(&self) -> Output {
        Output::from_result(self.render(|var, _| format!("unset {}", var)))
            .with_message("Cleared your sauce")
    }

    pub fn show(&self) -> Output {
        Output::from_message(self.render(|var, value| {
            format!(
                "export {}={}\n",
                var,
                shell_words::quote(&value.to_string())
            )
        }))
    }

    pub fn edit(&self) -> Output {
        Output::from_result(format!(
            "\"$EDITOR\" '{}'\n",
            self.context.sauce_path.to_string_lossy()
        ))
    }

    pub fn execute(&self) -> Output {
        Output::from_result(self.render(|var, value| format!("export {}={}\n", var, value)))
            .with_message(format!(
                "Sourced {}",
                self.context.sauce_path.to_string_lossy()
            ))
    }

    pub fn init(&self) -> Output {
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
        Output::from_result(statement)
    }

    pub fn create_subshell(&self) -> Output {
        Output::default()
    }
}
