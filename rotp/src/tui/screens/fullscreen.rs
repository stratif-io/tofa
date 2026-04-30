use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use rotp_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let entry = match vault.entries().get(state.selected_index) {
        Some(e) => e,
        None => return,
    };

    let code = generate_code_now(&entry.secret).unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now();
    let timer_color = theme::timer_color(secs);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Length(1), // name
            Constraint::Length(3), // code
            Constraint::Length(2), // progress bar
            Constraint::Length(1), // timer text
            Constraint::Min(1),
            Constraint::Length(1), // help
        ])
        .split(area);

    // Account name
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            entry.name.to_uppercase(),
            Style::default().fg(theme::DIM).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        chunks[1],
    );

    // Big code
    let spaced = format!("{}  {}", &code[..3], &code[3..]);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            spaced,
            Style::default()
                .fg(theme::GREEN)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        chunks[2],
    );

    // Progress bar (centered horizontally)
    let ratio = secs as f64 / 30.0;
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(Style::default().fg(timer_color).bg(theme::BG))
        .ratio(ratio);
    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(chunks[3]);
    f.render_widget(gauge, horiz[1]);

    // Timer text
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("expires in {}s", secs),
            Style::default().fg(timer_color),
        )))
        .alignment(Alignment::Center),
        chunks[4],
    );

    // Help bar
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ y ] copy   [ Esc ] back",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Center),
        chunks[6],
    );
}
