#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct TypingModeModel {
    pub sentence: String,
    pub answer: String,
    pub input_str: String,
}

impl TypingModeModel {
    pub fn new() -> Self {
        Self {
            sentence: String::from("test"),
            answer: String::from("test"),
            input_str: String::from(""),
        }
    }
    pub fn enter_char(&mut self, new_char: char) {
        self.input_str.insert(self.input_str.len(), new_char);
    }
}
