use crate::shell::utilities::{escape, qualify_binary_path};
use crate::shell::Shell;

pub struct Bash;

impl Shell for Bash {
    fn edit(&self, path: &str) -> String {
        format!("\"$EDITOR\" '{}'", path)
    }

    fn init(&self, binary: &str, autoload: bool) -> String {
        let mut init = format!(
            include_str!("bash_init.sh"),
            binary,
            qualify_binary_path(binary)
        );

        if autoload {
            init.push_str(&format!(include_str!("bash_init_autoload.sh"), binary));
        }

        init
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

#[cfg(test)]
mod tests {
    mod edit {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_edits_path() {
            let shell = Bash {};
            let output = shell.edit("foo/bar");
            assert_eq!(output, r#""$EDITOR" 'foo/bar'"#);
        }
    }

    mod init {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_defaults() {
            let shell = Bash {};
            let output = shell.init("foo", false);
            assert_eq!(
                output,
                "function foo {\n  eval \"$(command foo \"$@\")\"\n}\n"
            );
        }

        #[test]
        fn it_autoloads() {
            let shell = Bash {};
            let output = shell.init("foo", true);
            assert_eq!(output.contains("--autoload"), true);
        }
    }

    mod set_var {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Bash {};
            let output = shell.set_var("foo", "bar");
            assert_eq!(output, "export foo=bar");
        }
    }

    mod set_alias {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Bash {};
            let output = shell.set_alias("foo", "bar");
            assert_eq!(output, "alias foo=bar");
        }
    }

    mod set_function {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Bash {};
            let output = shell.set_function("foo", "bar");
            assert_eq!(output, "function foo {\n  bar\n}");
        }
    }

    mod unset_var {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Bash {};
            let output = shell.unset_var("foo");
            assert_eq!(output, "unset foo");
        }
    }

    mod unset_alias {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Bash {};
            let output = shell.unset_alias("foo");
            assert_eq!(output, "unalias foo 2>/dev/null");
        }
    }

    mod unset_function {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Bash {};
            let output = shell.unset_function("foo");
            assert_eq!(output, "unset -f foo");
        }
    }
}
