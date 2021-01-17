use crate::shell::utilities::escape;
use crate::shell::Shell;

pub struct Zsh;

impl Shell for Zsh {
    fn edit(&self, path: &str) -> String {
        format!("\"$EDITOR\" '{}'", path)
    }

    fn init(&self, binary: &str) -> String {
        format!(
            r#"
            sauce() {{
                eval "$(command {} "$@")"
            }}
            "#,
            binary,
        )
    }

    fn set_var(&self, var: &str, value: &str) -> String {
        format!("export {}={}", var, escape(value))
    }

    fn set_alias(&self, var: &str, value: &str) -> String {
        format!("alias {}={}", var, escape(&value))
    }

    fn set_function(&self, var: &str, value: &str) -> String {
        format!("function {} {{\n  {}\n}}", var, value.replace("\n", "\n  "))
    }

    fn unset_var(&self, var: &str) -> String {
        format!("unset {}", var)
    }

    fn unset_alias(&self, var: &str) -> String {
        format!("unalias {} 2>/dev/null", var)
    }

    fn unset_function(&self, var: &str) -> String {
        format!("unset -f {}", var)
    }
}
