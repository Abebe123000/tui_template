use crate::app::{App, AppResult};
use crate::app_mode::AppMode;
use crate::typing_mode_model::TypingModeModel;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Other handlers you could add here.
        _ => {
            let app_mode = app.mode.clone();
            match app_mode {
                AppMode::Typing(model) => {
                    handle_key_events_for_type(key_event, app, model)?;
                }
            }
        }
    }
    Ok(())
}

fn handle_key_events_for_type(
    key_event: KeyEvent,
    app: &mut App,
    model: TypingModeModel,
) -> AppResult<()> {
    let mut new_model = model.clone();
    match key_event.code {
        KeyCode::Char(new_char) => {
            new_model.enter_char(new_char);
            app.mode = AppMode::Typing(new_model);
        }
        _ => {}
    }
    Ok(())
}
