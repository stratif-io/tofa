use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use rotp_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);
    let entries = vault.entries();
    let item_w = chunks[0].width as usize;

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let selected = i == state.selected_index;

            let (issuer, account) = if let Some(pos) = entry.name.find(':') {
                (&entry.name[..pos], &entry.name[pos + 1..])
            } else {
                (entry.name.as_str(), "")
            };

            let show = state.show_codes || selected;
            let code_str = if show {
                let code = generate_code_now(&entry.secret)
                    .unwrap_or_else(|_| "000000".to_string());
                format!("{} {}", &code[..3], &code[3..])
            } else {
                "••• •••".to_string()
            };
            let code_col = if selected { timer_col } else { theme::MUTED };

            let border_char = if selected { "▌ " } else { "  " };
            let border_col  = if selected { theme::ACCENT } else { theme::BG };

            let pad_len = item_w
                .saturating_sub(2)
                .saturating_sub(issuer.len())
                .saturating_sub(code_str.len())
                .saturating_sub(2);
            let padding = " ".repeat(pad_len);

            let issuer_col = if selected { theme::TEXT } else { theme::DIM };
            let issuer_mod = if selected { Modifier::BOLD } else { Modifier::empty() };

            let line0 = Line::from(vec![
                Span::styled(border_char, Style::default().fg(border_col)),
                Span::styled(issuer, Style::default().fg(issuer_col).add_modifier(issuer_mod)),
                Span::raw(padding),
                Span::raw("  "),
                Span::styled(
                    code_str,
                    Style::default().fg(code_col).add_modifier(
                        if selected { Modifier::BOLD } else { Modifier::empty() }
                    ),
                ),
            ]);

            let line1 = if account.is_empty() {
                Line::from(Span::raw(""))
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(account, Style::default().fg(theme::MUTED)),
                ])
            };

            let bar_w = item_w.saturating_sub(2);
            let filled = ((secs as usize * bar_w) / 30).min(bar_w);
            let bar = format!(
                "  {}{}",
                "█".repeat(filled),
                "░".repeat(bar_w.saturating_sub(filled))
            );
            let bar_col = if selected { timer_col } else { theme::MUTED };
            let line2 = Line::from(Span::styled(bar, Style::default().fg(bar_col)));

            ListItem::new(Text::from(vec![line0, line1, line2]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().style(Style::default().bg(theme::BG)))
        .highlight_style(Style::default().bg(theme::SELECTED));

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let toggle = if state.show_codes { "hide" } else { "show" };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("↑↓ navigate · Enter fullscreen · h {toggle} · a add · i detail · e export · d delete · y copy · q quit"),
            Style::default().fg(theme::MUTED),
        )))
        .style(Style::default().bg(theme::BG)),
        chunks[1],
    );

    // Toast overlay for copy confirmation
    if let Some(msg) = &state.status_message {
        let is_copy = msg.contains("Copied") || msg.contains("copied");
        let (border_col, text_col) = if is_copy {
            (theme::ACCENT, theme::ACCENT)
        } else {
            (theme::URGENT, theme::URGENT)
        };
        let label = if is_copy { format!("  ✓  {}  ", msg) } else { format!("  {}  ", msg) };
        let toast_w = (label.chars().count() as u16 + 2).min(area.width);
        let toast_h = 3u16;
        let toast = Rect {
            x: area.x + (area.width.saturating_sub(toast_w)) / 2,
            y: area.y + (area.height.saturating_sub(toast_h)) / 2,
            width: toast_w,
            height: toast_h,
        };
        f.render_widget(Clear, toast);
        f.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_col))
                .style(Style::default().bg(theme::BG)),
            toast,
        );
        let inner = Rect { x: toast.x + 1, y: toast.y + 1, width: toast.width.saturating_sub(2), height: 1 };
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                label,
                Style::default().fg(text_col).add_modifier(Modifier::BOLD),
            )))
            .alignment(Alignment::Center),
            inner,
        );
    }
}
