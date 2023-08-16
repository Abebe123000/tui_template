use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
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
    pub message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            state: AppState::StateA,
            message: String::from("aaa"),
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
        &self.message
    }

    pub fn add_message(&mut self, str: &str) {
        self.message = self.message.clone() + str;
    }

    pub fn change_state_to_a(&mut self) {
        self.state = AppState::StateA;
    }

    pub fn change_state_to_b(&mut self) {
        self.state = AppState::StateB;
    }
}
