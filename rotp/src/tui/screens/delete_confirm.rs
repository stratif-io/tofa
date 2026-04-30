use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rotp_core::store::Vault;
use crate::tui::{state::AppState, theme};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let entry_name = vault
        .entries()
        .get(state.selected_index)
        .map(|e| e.name.as_str())
        .unwrap_or("this entry");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(area);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    let content = vec![
        Line::from(Span::styled(
            "⚠  Delete this account?",
            Style::default().fg(theme::RED).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("{} will be permanently", entry_name),
            Style::default().fg(theme::DIM),
        )),
        Line::from(Span::styled(
            "removed from the vault.",
            Style::default().fg(theme::DIM),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[ y ] Yes   ",
                Style::default().fg(theme::RED).add_modifier(Modifier::BOLD),
            ),
            Span::styled("[ n ] No", Style::default().fg(theme::DIM)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme::RED))
                    .style(Style::default().bg(theme::BG)),
            ),
        horiz[1],
    );
}
