use std::{
    cell::RefCell,
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
    str,
};

use crate::{output::Output, shell::Shell, Context};

pub fn mkpath(path: &str) -> PathBuf {
    Path::new(path).canonicalize().unwrap()
}

pub struct TestShell;

impl Shell for TestShell {
    fn name(&self) -> &'static str {
        "test"
    }

    fn init(&self, binary: &str, autoload_hook: bool) -> String {
        if autoload_hook {
            format!("{} {}", binary, "--autoload")
        } else {
            binary.to_string()
        }
    }

    fn set_var(&self, var: &str, value: &str) -> String {
        format!("export {}={}", var, value)
    }

    fn set_alias(&self, var: &str, value: &str) -> String {
        format!("alias {}={}", var, value)
    }

    fn set_function(&self, var: &str, value: &str) -> String {
        format!("function {}={}", var, value)
    }

    fn unset_var(&self, var: &str) -> String {
        format!("unset {}", var)
    }

    fn unset_alias(&self, var: &str) -> String {
        format!("unalias {}", var)
    }

    fn unset_function(&self, var: &str) -> String {
        format!("unset {}", var)
    }
}

#[derive(Clone)]
pub struct MockWriter(Rc<RefCell<Vec<u8>>>);

impl Default for MockWriter {
    fn default() -> Self {
        Self {
            0: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl MockWriter {
    pub fn value(&self) -> String {
        let data = &self.0.borrow();
        String::from_utf8(data.to_vec()).unwrap()
    }
}

impl Write for MockWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.borrow_mut().flush()
    }
}

pub fn setup<'a>() -> (MockWriter, MockWriter, Context<'a>) {
    let out = MockWriter::default();
    let err = MockWriter::default();

    let context = Context {
        output: Output::new(Box::new(out.clone()), Box::new(err.clone())),
        ..Default::default()
    };

    (out, err, context)
}
