use crate::shell::utilities::{escape, qualify_binary_path};
use crate::shell::Shell;

pub struct Fish;

impl Shell for Fish {
    fn name(&self) -> &'static str {
        "fish"
    }

    fn init(&self, binary: &str, autoload_hook: bool) -> String {
        let mut init = format!(
            include_str!("fish_init.fish"),
            binary,
            qualify_binary_path(binary)
        );

        if autoload_hook {
            init.push_str(&format!(include_str!("fish_init_autoload.fish"), binary));
        }

        init
    }

    fn set_var(&self, var: &str, value: &str) -> String {
        format!("set -x {} {}", var, escape(value))
    }

    fn set_alias(&self, var: &str, value: &str) -> String {
        format!("alias {} {}", var, escape(&value))
    }

    fn set_function(&self, var: &str, value: &str) -> String {
        format!("function {}\n  {}\nend", var, value.replace("\n", "\n  "))
    }

    fn unset_var(&self, var: &str) -> String {
        format!("set -e {}", var)
    }

    fn unset_alias(&self, var: &str) -> String {
        format!("functions --erase {}", var)
    }

    fn unset_function(&self, var: &str) -> String {
        format!("functions --erase {}", var)
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
            let shell = Fish {};
            let output = shell.edit(Some(OsString::from("foo")), "foo/bar");
            assert_eq!(output, Some(r#"foo 'foo/bar'"#.to_string()));
        }
    }

    mod init {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_defaults() {
            let shell = Fish {};
            let output = shell.init("foo", false);
            assert_eq!(
                output,
                "function foo\n  command foo --shell fish $argv | source\nend\n"
            );
        }

        #[test]
        fn it_includes_autoload() {
            let shell = Fish {};
            let output = shell.init("foo", true);
            assert_eq!(output.contains("--autoload"), true);
        }
    }

    mod set_var {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Fish {};
            let output = shell.set_var("foo", "bar");
            assert_eq!(output, "set -x foo bar");
        }
    }

    mod set_alias {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Fish {};
            let output = shell.set_alias("foo", "bar");
            assert_eq!(output, "alias foo bar");
        }
    }

    mod set_function {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Fish {};
            let output = shell.set_function("foo", "bar");
            assert_eq!(output, "function foo\n  bar\nend");
        }
    }

    mod unset_var {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Fish {};
            let output = shell.unset_var("foo");
            assert_eq!(output, "set -e foo");
        }
    }

    mod unset_alias {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Fish {};
            let output = shell.unset_alias("foo");
            assert_eq!(output, "functions --erase foo");
        }
    }

    mod unset_function {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works() {
            let shell = Fish {};
            let output = shell.unset_function("foo");
            assert_eq!(output, "functions --erase foo");
        }
    }
}
