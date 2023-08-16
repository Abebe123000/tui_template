use crate::app::{App, AppResult};
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
        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        KeyCode::Char('a') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.change_state_to_a();
        }
        KeyCode::Char('b') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.change_state_to_b();
        }
        // Other handlers you could add here.
        _ => {
            handle_key_events_for_type(key_event, app)?;
        }
    }
    Ok(())
}

fn handle_key_events_for_type(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('a') => {
            app.add_message("a");
        }
        KeyCode::Char('b') => {
            app.add_message("b");
        }
        KeyCode::Char('z') => {
            app.add_message("z");
        }
        _ => {}
    }
    Ok(())
}
