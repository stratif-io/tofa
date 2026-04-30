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

    let code = generate_code_now(&state.add_parsed_secret)
        .unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now();
    let timer_color = theme::timer_color(secs);

    let error_line = if let Some(msg) = &state.status_message {
        Line::from(Span::styled(msg.as_str(), Style::default().fg(theme::RED)))
    } else {
        Line::from("")
    };

    let box_height = 10u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(52);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let inner = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    let content = vec![
        Line::from(Span::styled(
            "Add OTP",
            Style::default().fg(theme::GREEN).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled(
                format!("{}  {}", &code[..3], &code[3..]),
                Style::default().fg(theme::GREEN).add_modifier(Modifier::BOLD),
            ),
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
        inner,
    );
}
