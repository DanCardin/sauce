pub struct Output {
    results: Vec<String>,
    messages: Vec<String>,
}

impl Output {
    #[allow(unused)]
    pub fn from_result(result: String) -> Self {
        let mut output = Self::default();
        output.with_result(result);
        output
    }

    #[allow(unused)]
    pub fn from_message(message: String) -> Self {
        let mut output = Self::default();
        output.with_message(message);
        output
    }

    pub fn with_result<F: Into<String>>(&mut self, result: F) -> &mut Self {
        self.results.push(result.into());
        self
    }

    pub fn with_message<F: Into<String>>(&mut self, message: F) -> &mut Self {
        self.messages.push(message.into());
        self
    }

    pub fn push_result<F: Into<String>>(&mut self, result: F) {
        self.results.push(result.into());
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
