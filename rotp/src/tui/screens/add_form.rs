use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crate::tui::{state::AppState, theme};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Length(14), Constraint::Min(0)])
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

    let name_color = if state.add_focused_field == 0 { theme::GREEN } else { theme::DIM };
    let secret_color = if state.add_focused_field == 1 { theme::GREEN } else { theme::DIM };

    let content = vec![
        Line::from(Span::styled(
            "Add OTP",
            Style::default().fg(theme::GREEN).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Name:", Style::default().fg(theme::DIM))),
        Line::from(Span::styled(
            format!("{}_", state.add_name),
            Style::default().fg(name_color),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "QR code path, otpauth:// URI, or Base32 secret:",
            Style::default().fg(theme::DIM),
        )),
        Line::from(Span::styled(
            format!("{}_", state.add_secret_input),
            Style::default().fg(secret_color),
        )),
        Line::from(""),
        error_line,
        Line::from(""),
        Line::from(Span::styled(
            "[ Tab ] switch field   [ Enter ] confirm   [ Esc ] cancel",
            Style::default().fg(theme::DIM),
        )),
    ];

    f.render_widget(
        Paragraph::new(content).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::DIM_GREEN))
                .style(Style::default().bg(theme::BG)),
        ),
        horiz[1],
    );
}
