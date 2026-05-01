use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::time::SystemTime;
use tofa_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);

    // Use the selected entry's period for the header timer, fall back to 30s
    let selected_entry = vault.entries().get(state.selected_index);
    let header_secs = selected_entry.map(|e| seconds_remaining_now(e)).unwrap_or(30);

    render_header(f, chunks[0], vault, header_secs);
    render_list(f, chunks[1], state, vault);
    render_footer(f, chunks[2]);
    super::toast::render(f, area, state);
}

fn render_header(f: &mut Frame, area: Rect, vault: &Vault, secs: u64) {
    let timer_col = theme::timer_color(secs);
    let count = vault.entries().len();

    let left = Line::from(vec![
        Span::styled("tofa", Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(format!("[{}]", count), Style::default().fg(theme::ACCENT).bg(theme::BADGE_BG)),
    ]);
    let right = Line::from(vec![
        Span::styled(format!("{}s", secs), Style::default().fg(timer_col)),
    ]);

    f.render_widget(
        Paragraph::new(left).style(Style::default().bg(theme::SURFACE)),
        Rect { x: area.x, y: area.y, width: area.width / 2, height: 1 },
    );
    f.render_widget(
        Paragraph::new(right).alignment(Alignment::Right).style(Style::default().bg(theme::SURFACE)),
        Rect { x: area.x + area.width / 2, y: area.y, width: area.width - area.width / 2, height: 1 },
    );
    f.render_widget(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::SURFACE)),
        Rect { x: area.x, y: area.y, width: area.width, height: 2 },
    );
}

fn render_list(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let entries = vault.entries();

    let labels: Vec<String> = entries.iter().map(|entry| {
        if let Some(pos) = entry.name.find(':') {
            let issuer  = &entry.name[..pos];
            let account = &entry.name[pos + 1..];
            if account.is_empty() { entry.name.clone() } else { format!("{} · {}", issuer, account) }
        } else {
            entry.name.clone()
        }
    }).collect();

    let max_label_w = labels.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let code_col_offset = 2 + max_label_w + 2;

    const BAR_LEN: usize = 20;

    let now_ms = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let selected = i == state.selected_index;
            let label = &labels[i];
            let secs = seconds_remaining_now(entry);
            let timer_col = theme::timer_color(secs);

            let show = state.show_codes || selected;
            let code_str = if show {
                let code = generate_code_now(entry).unwrap_or_else(|_| "000000".to_string());
                format!("{} {}", &code[..3], &code[3..])
            } else {
                "••• •••".to_string()
            };

            let (cursor, label_col, label_mod, code_col) = if selected {
                ("› ", theme::TEXT, Modifier::BOLD, timer_col)
            } else {
                ("  ", theme::DIM, Modifier::empty(), if state.show_codes { theme::DIM } else { theme::MUTED })
            };

            let bar_col = if selected { timer_col } else { theme::MUTED };
            let period_ms = entry.period as u64 * 1000;
            let ms_into_period = now_ms % period_ms;
            let ms_left = period_ms - ms_into_period;
            let filled = ((ms_left * BAR_LEN as u64) / period_ms) as usize;
            let expiry_bar = if show {
                format!(" {}{}", "█".repeat(filled), "░".repeat(BAR_LEN - filled))
            } else {
                String::new()
            };

            let pad = max_label_w.saturating_sub(label.chars().count());

            let content = Line::from(vec![
                Span::styled(cursor, Style::default().fg(theme::ACCENT)),
                Span::styled(label.clone(), Style::default().fg(label_col).add_modifier(label_mod)),
                Span::raw(" ".repeat(pad + 2)),
                Span::styled(code_str, Style::default().fg(code_col)),
                Span::styled(expiry_bar, Style::default().fg(bar_col)),
            ]);

            let separator = Line::from(Span::styled(
                "─".repeat(code_col_offset + 9 + BAR_LEN + 1),
                Style::default().fg(theme::BORDER),
            ));

            ListItem::new(vec![content, separator])
        })
        .collect();

    let list = List::new(items).block(Block::default().style(Style::default().bg(theme::BG)));
    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_footer(f: &mut Frame, area: Rect) {
    f.render_widget(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::BG)),
        area,
    );
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "↑↓ nav  spc fullscreen  i detail  h codes  a add  d del  y copy  e export  q quit",
            Style::default().fg(theme::MUTED),
        )))
        .style(Style::default().bg(theme::BG)),
        Rect { x: area.x, y: area.y + 1, width: area.width, height: 1 },
    );
}
