use std::fmt::Display;

use ansi_term::{ANSIString, ANSIStrings};

#[derive(Debug)]
pub struct Output<'a> {
    results: Vec<String>,
    messages: Vec<ANSIStrings<'a>>,
    code: Option<ErrorCode>,
}

impl<'a> Output<'a> {
    pub fn push_result<F: Into<String>>(&mut self, result: F) {
        self.results.push(result.into());
    }

    pub fn push_message(&mut self, message: &'a [ANSIString]) {
        self.messages.push(ANSIStrings(message));
    }

    pub fn push_error(&mut self, code: ErrorCode, message: &'a [ANSIString]) {
        self.code = Some(code);
        self.messages.push(ANSIStrings(message));
    }

    pub fn result(&self) -> String {
        self.results.join("\n") + "\n"
    }

    pub fn message(&self) -> String {
        self.messages
            .iter()
            .map(|m| format!("{}\n", m))
            .collect::<Vec<String>>()
            .join("")
        // self.messages.join("\n") + "\n"
    }

    pub fn error_code(&self) -> Option<i32> {
        self.code.clone().map(|c| c as i32)
    }
}

impl<'a> Default for Output<'a> {
    fn default() -> Self {
        Self {
            results: Vec::new(),
            messages: Vec::new(),
            code: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorCode {
    WriteError = 1,
    ParseError = 2,
}
