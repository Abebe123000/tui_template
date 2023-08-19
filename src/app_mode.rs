use crate::typing_mode_model::TypingModeModel;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum AppMode {
    Typing(TypingModeModel),
}
