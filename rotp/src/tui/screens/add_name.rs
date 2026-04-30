use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Length(10), Constraint::Min(0)])
        .split(area);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(chunks[1]);

    let error_line = if let Some(msg) = &state.status_message {
        Line::from(Span::styled(msg.as_str(), Style::default().fg(theme::RED)))
    } else {
        Line::from("")
    };

    let content = vec![
        Line::from(Span::styled(
            "Add OTP",
            Style::default().fg(theme::GREEN).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Name for this account:", Style::default().fg(theme::DIM))),
        Line::from(Span::styled(
            format!("{}_", state.add_name),
            Style::default().fg(theme::GREEN),
        )),
        Line::from(""),
        error_line,
        Line::from(""),
        Line::from(Span::styled(
            "[ Enter ] save   [ Esc ] back",
            Style::default().fg(theme::DIM),
        )),
    ];

    f.render_widget(
        Paragraph::new(content).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::GREEN))
                .style(Style::default().bg(theme::BG)),
        ),
        horiz[1],
    );
}
