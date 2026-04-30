use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rotp_core::totp::{generate_code_now, seconds_remaining_now};

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

    let code = generate_code_now(&state.add_parsed_secret)
        .unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now();
    let timer_color = theme::timer_color(secs);

    let code_display = format!("{}  {}", &code[..3], &code[3..]);

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
        Line::from(vec![
            Span::styled(code_display, Style::default().fg(theme::GREEN).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(format!("⏱ {}s", secs), Style::default().fg(timer_color)),
        ]),
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
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme::GREEN))
                    .style(Style::default().bg(theme::BG)),
            ),
        horiz[1],
    );
}
