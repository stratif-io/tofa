use crate::tui::state::AppState;
use tofa_theme::palette as theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use tofa_core::store::Vault;

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let box_h = area.height.saturating_sub(4).max(8);
    let box_w = area.width.min(60);
    let pad_x = (area.width.saturating_sub(box_w)) / 2;
    let pad_y = (area.height.saturating_sub(box_h)) / 2;
    let modal = Rect { x: area.x + pad_x, y: area.y + pad_y, width: box_w, height: box_h };

    f.render_widget(Clear, modal);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BRAND))
            .style(Style::default().bg(theme::BG)),
        modal,
    );

    let inner = Rect {
        x: modal.x + 1,
        y: modal.y + 1,
        width: modal.width.saturating_sub(2),
        height: modal.height.saturating_sub(2),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Export OTPs",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        ))),
        chunks[0],
    );

    let entries = vault.entries();
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let checked  = state.export_checked.get(i).copied().unwrap_or(true);
            let selected = i == state.export_selected;
            let checkbox = if checked { "[✓]" } else { "[ ]" };
            let row_style = if selected {
                Style::default().fg(theme::BRAND).bg(theme::SURFACE).add_modifier(Modifier::BOLD)
            } else if checked {
                Style::default().fg(theme::TEXT)
            } else {
                Style::default().fg(theme::TEXT_MUTED)
            };
            let cb_style = if checked { row_style.fg(theme::BRAND) } else { row_style };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{checkbox} "), cb_style),
                Span::styled(entry.name.clone(), row_style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.export_selected));
    f.render_stateful_widget(
        List::new(items).block(Block::default().style(Style::default().bg(theme::BG))),
        chunks[2],
        &mut list_state,
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ Space ] toggle  [ Enter ] generate QR  [ Esc ] back",
            Style::default().fg(theme::TEXT_MUTED),
        ))),
        chunks[3],
    );
}
