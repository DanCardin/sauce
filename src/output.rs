pub struct Output {
    results: Vec<String>,
    messages: Vec<String>,
}

impl Output {
    pub fn from_result(result: String) -> Self {
        Self::default().with_result(result)
    }

    pub fn from_message(message: String) -> Self {
        Self::default().with_message(message)
    }

    pub fn with_result<F: Into<String>>(mut self, result: F) -> Self {
        self.results.push(result.into());
        self
    }

    pub fn with_message<F: Into<String>>(mut self, message: F) -> Self {
        self.messages.push(message.into());
        self
    }

    pub fn push_message<F: Into<String>>(&mut self, message: F) {
        self.messages.push(message.into());
    }

    pub fn result(&self) -> String {
        self.results.join("\n") + "\n"
    }

    pub fn message(&self) -> String {
        self.messages.join("\n") + "\n"
    }
}

impl Default for Output {
    fn default() -> Self {
        Self {
            results: Vec::new(),
            messages: Vec::new(),
        }
    }
}
