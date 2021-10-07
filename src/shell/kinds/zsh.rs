use crate::shell::utilities::{escape, qualify_binary_path};
use crate::shell::Shell;

pub struct Zsh;

impl Shell for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }

    fn init(&self, binary: &str, autoload_hook: bool) -> String {
        let mut init = format!(
            include_str!("zsh_init.zsh"),
            binary,
            qualify_binary_path(binary)
        );

        if autoload_hook {
            init.push_str(&format!(include_str!("zsh_init_autoload.zsh"), binary));
        }

        init
    }

    fn set_var(&self, var: &str, value: &str) -> String {
        format!("export {}={}", var, escape(value))
    }

    fn set_alias(&self, var: &str, value: &str) -> String {
        format!("alias {}={}", var, escape(value))
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
        use std::ffi::OsString;

        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_edits_path() {
            let shell = Zsh {};
            let output = shell.edit(Some(OsString::from("foo")), "foo/bar");
            assert_eq!(output, Some(r#"foo 'foo/bar'"#.to_string()));
        }
    }

    mod init {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_defaults() {
            let shell = Zsh {};
            let output = shell.init("foo", false);
            assert_eq!(
                output,
                "function foo {\n  eval \"$(command foo --shell zsh \"$@\")\"\n}\n"
            );
        }

        #[test]
        fn it_includes_autoload() {
            let shell = Zsh {};
            let output = shell.init("foo", true);
            assert_eq!(output.contains("--autoload"), true);
        }
    }

    mod set_var {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Zsh {};
            let output = shell.set_var("foo", "bar");
            assert_eq!(output, "export foo=bar");
        }
    }

    mod set_alias {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Zsh {};
            let output = shell.set_alias("foo", "bar");
            assert_eq!(output, "alias foo=bar");
        }
    }

    mod set_function {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Zsh {};
            let output = shell.set_function("foo", "bar");
            assert_eq!(output, "function foo {\n  bar\n}");
        }
    }

    mod unset_var {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Zsh {};
            let output = shell.unset_var("foo");
            assert_eq!(output, "unset foo");
        }
    }

    mod unset_alias {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Zsh {};
            let output = shell.unset_alias("foo");
            assert_eq!(output, "unalias foo 2>/dev/null");
        }
    }

    mod unset_function {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Zsh {};
            let output = shell.unset_function("foo");
            assert_eq!(output, "unset -f foo");
        }
    }
}
