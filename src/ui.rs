use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app_mode::AppMode;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let app_mode = app.mode.clone();
    match app_mode {
        AppMode::Typing(model) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Percentage(80),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(frame.size());
            frame.render_widget(
                Paragraph::new(model.sentence)
                    .block(
                        Block::default()
                            .title("Sentence")
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                    .alignment(Alignment::Center),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(model.input_str)
                    .block(
                        Block::default()
                            .title("input")
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                    .alignment(Alignment::Center),
                chunks[2],
            )
        }
    }
}
