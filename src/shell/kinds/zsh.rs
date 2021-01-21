use crate::shell::utilities::escape;
use crate::shell::Shell;

pub struct Zsh;

impl Shell for Zsh {
    fn edit(&self, path: &str) -> String {
        format!("\"$EDITOR\" '{}'", path)
    }

    fn init(&self, binary: &str, autoload_hook: bool) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            r#"function {0} {{ eval "$(command {1}{0} "$@")" }}"#,
            binary,
            if cfg!(debug_assertions) {
                "./target/debug/"
            } else {
                ""
            }
        ));

        if autoload_hook {
            parts.push(format!(
                "function _{0}_autoload {{ {0} --autoload }}",
                binary
            ));
            parts.push(format!("add-zsh-hook chpwd _{}_autoload", binary));
        }

        parts.join("\n")
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
