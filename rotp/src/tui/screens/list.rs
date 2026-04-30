use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
    Frame,
};
use rotp_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};
use crate::tui::{state::AppState, theme};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    // Header
    let count = vault.entries().len();
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "rotp",
            Style::default().fg(theme::GREEN).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  —  {} account{}", count, if count == 1 { "" } else { "s" }),
            Style::default().fg(theme::DIM),
        ),
    ]))
    .style(Style::default().bg(theme::BG));
    f.render_widget(header, chunks[0]);

    // Entry list
    let secs = seconds_remaining_now();
    let timer_color = theme::timer_color(secs);

    let items: Vec<ListItem> = vault
        .entries()
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let is_selected = i == state.selected_index;
            if is_selected {
                let code = generate_code_now(&entry.secret)
                    .unwrap_or_else(|_| "------".to_string());
                ListItem::new(Line::from(vec![
                    Span::styled("▶ ", Style::default().fg(theme::GREEN)),
                    Span::styled(
                        entry.name.clone(),
                        Style::default()
                            .fg(theme::GREEN)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("  "),
                    Span::styled(
                        format!("{} {}", &code[..3], &code[3..]),
                        Style::default()
                            .fg(theme::WHITE)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("  "),
                    Span::styled(
                        format!("⏱ {}s", secs),
                        Style::default().fg(timer_color),
                    ),
                ]))
            } else {
                ListItem::new(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(entry.name.clone(), Style::default().fg(theme::DIM)),
                    Span::raw("  "),
                    Span::styled("••• •••", Style::default().fg(theme::DIM)),
                ]))
            }
        })
        .collect();

    let list = List::new(items).block(Block::default().style(Style::default().bg(theme::BG)));
    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    f.render_stateful_widget(list, chunks[1], &mut list_state);

    // Help/status bar
    let bar = if let Some(msg) = &state.status_message {
        Span::styled(msg.as_str(), Style::default().fg(theme::ORANGE))
    } else {
        Span::styled(
            "[ ↑↓ ] navigate  [ Enter ] fullscreen  [ a ] add  [ d ] delete  [ y ] copy  [ q ] quit",
            Style::default().fg(theme::DIM),
        )
    };
    f.render_widget(
        Paragraph::new(Line::from(bar)).style(Style::default().bg(theme::BG)),
        chunks[2],
    );
}
