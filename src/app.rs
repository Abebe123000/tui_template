use crate::app::AppState::StateA;
use crate::app::AppState::StateB;
use std::collections::HashMap;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum AppState {
    StateA,
    StateB,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub state: AppState,
    pub message_map: HashMap<AppState, String>,
}

impl Default for App {
    fn default() -> Self {
        let mut message_map = HashMap::new();
        message_map.insert(StateA, String::from("aaa"));
        message_map.insert(StateB, String::from("bbb"));
        Self {
            running: true,
            counter: 0,
            state: AppState::StateA,
            message_map: message_map,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn get_message(&self) -> &str {
        match self.message_map.get(&self.state) {
            Some(string) => &string,
            _ => "",
        }
    }

    pub fn add_message(&mut self, str: &str) {
        if let Some(string) = self.message_map.get(&self.state) {
            self.message_map
                .insert(self.state, string.to_owned() + &str);
        }
    }

    pub fn change_state_to_a(&mut self) {
        self.state = AppState::StateA;
    }

    pub fn change_state_to_b(&mut self) {
        self.state = AppState::StateB;
    }
}
