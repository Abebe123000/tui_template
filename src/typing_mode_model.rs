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

    /// input_str が answer と同じになり得る入力のみ input_str に入力する
    ///
    /// 例
    /// answer : test
    /// input_str : te
    /// の場合
    /// s であれば s を input_str に追加する
    /// それ以外であれば何もしない
    pub fn enter_char(&mut self, new_char: char) {
        let input_str_len = self.input_str.len();
        let inputable_char_option = self.answer.chars().nth(input_str_len);
        if inputable_char_option == Some(new_char) {
            self.input_str.insert(self.input_str.len(), new_char);
        }
    }
}
