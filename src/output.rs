use anyhow::Result;
use std::{
    fmt::Display,
    io::{stderr, stdout, Write},
    ops::Deref,
};

use ansi_term::{ANSIString, ANSIStrings};

pub struct Output {
    out: Box<dyn Write>,
    err: Box<dyn Write>,
    color_enabled: bool,
    quiet: bool,
    verbose: bool,
    code: Option<ErrorCode>,
}

impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Output")
            .field("color_enabled", &self.color_enabled)
            .field("code", &self.code)
            .finish()
    }
}

impl Output {
    pub fn new(
        out: Box<dyn Write>,
        err: Box<dyn Write>,
        color_enabled: bool,
        quiet: bool,
        verbose: bool,
    ) -> Self {
        Self {
            out,
            err,
            color_enabled,
            quiet,
            verbose,
            code: None,
        }
    }

    fn format(&self, output: impl Display) -> String {
        let mut result = format!("{}", output);
        if !result.is_empty() {
            result.push('\n');
        }
        result
    }

    pub fn set_quiet(&mut self, value: bool) {
        self.quiet = value;
    }

    pub fn set_verbose(&mut self, value: bool) {
        self.verbose = value;
    }

    pub fn output(&mut self, output: impl Display) -> bool {
        let data = self.format(output);
        let result = self.out.write_all(data.as_bytes()).is_ok();
        if self.verbose {
            self.err
                .write_all(data.as_bytes())
                .expect("Couldn't write verbose output");
        }
        result
    }

    pub fn notify(&mut self, message: &[ANSIString]) -> bool {
        let message = if self.color_enabled {
            self.format(ANSIStrings(message))
        } else {
            self.format(
                message
                    .iter()
                    .map(|f| f.deref())
                    .collect::<Vec<&str>>()
                    .join(""),
            )
        };

        self.notify_str(&message)
    }

    pub fn notify_str(&mut self, message: &str) -> bool {
        if !self.quiet {
            self.err.write_all(message.as_bytes()).is_ok()
        } else {
            true
        }
    }

    pub fn notify_error(&mut self, code: ErrorCode, message: &[ANSIString]) -> bool {
        self.code = Some(code);
        self.notify(message)
    }

    pub fn error_code(&self) -> Option<i32> {
        self.code.clone().map(|c| c as i32)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.out.flush()?;
        self.err.flush()?;
        Ok(())
    }
}

impl Default for Output {
    fn default() -> Self {
        Self {
            out: Box::new(stdout()),
            err: Box::new(stderr()),
            color_enabled: true,
            quiet: false,
            verbose: false,
            code: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorCode {
    WriteError = 1,
    ParseError = 2,
}
