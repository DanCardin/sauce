#[derive(Debug)]
pub struct Output {
    results: Vec<String>,
    messages: Vec<String>,
    code: Option<ErrorCode>,
}

impl Output {
    pub fn push_result<F: Into<String>>(&mut self, result: F) {
        self.results.push(result.into());
    }

    pub fn push_message<F: Into<String>>(&mut self, message: F) {
        self.messages.push(message.into());
    }

    pub fn push_error<F: Into<String>>(&mut self, code: ErrorCode, message: F) {
        self.code = Some(code);
        self.messages.push(message.into());
    }

    pub fn result(&self) -> String {
        self.results.join("\n") + "\n"
    }

    pub fn message(&self) -> String {
        self.messages.join("\n") + "\n"
    }

    pub fn error_code(&self) -> Option<i32> {
        self.code.clone().map(|c| c as i32)
    }
}

impl Default for Output {
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
