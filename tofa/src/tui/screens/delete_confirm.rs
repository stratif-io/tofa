use crate::theme::palette as theme;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use tofa_core::store::Vault;

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let entry_name = vault
        .entries()
        .get(state.selected_index)
        .map(|e| e.name.as_str())
        .unwrap_or("this entry");

    let box_height = 9u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(48);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let inner = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    let content = vec![
        Line::from(Span::styled(
            "Delete this account?",
            Style::default()
                .fg(theme::DANGER)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("{} will be permanently", entry_name),
            Style::default().fg(theme::TEXT_MUTED),
        )),
        Line::from(Span::styled(
            "removed from the vault.",
            Style::default().fg(theme::TEXT_MUTED),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[ y ] Yes   ",
                Style::default()
                    .fg(theme::DANGER)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("[ n ] No", Style::default().fg(theme::TEXT_MUTED)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(content).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::DANGER))
                .style(Style::default().bg(theme::BG)),
        ),
        inner,
    );
}
