use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(area);

    let masked: String = "•".repeat(state.passphrase_input.len());
    let error_line = if state.unlock_error {
        Line::from(Span::styled(
            "Wrong passphrase. Try again.",
            Style::default().fg(theme::RED),
        ))
    } else {
        Line::from("")
    };

    let content = Paragraph::new(vec![
        Line::from(Span::styled(
            "r o t p",
            Style::default()
                .fg(theme::GREEN)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled("OTP Manager", Style::default().fg(theme::DIM))),
        Line::from(""),
        Line::from(Span::styled("Passphrase:", Style::default().fg(theme::DIM))),
        Line::from(Span::styled(masked, Style::default().fg(theme::GREEN))),
        error_line,
        Line::from(""),
        Line::from(Span::styled(
            "[ Enter ] unlock   [ Ctrl+C ] quit",
            Style::default().fg(theme::DIM),
        )),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::DIM_GREEN))
            .style(Style::default().bg(theme::BG)),
    );

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    f.render_widget(content, horiz[1]);
}
