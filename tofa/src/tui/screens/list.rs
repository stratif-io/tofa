use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use tofa_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    // Fond global
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    // Layout : header(1) + liste(min) + footer(1)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header + séparateur
            Constraint::Min(1),    // liste
            Constraint::Length(2), // séparateur + footer
        ])
        .split(area);

    let secs = seconds_remaining_now();

    render_header(f, chunks[0], vault, secs);
    render_list(f, chunks[1], state, vault, secs);
    render_footer(f, chunks[2]);
    render_toast(f, area, state);
}

fn render_header(f: &mut Frame, area: Rect, vault: &Vault, secs: u64) {
    let timer_col = theme::timer_color(secs);
    let count = vault.entries().len();

    let left = Line::from(vec![
        Span::styled("tofa", Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(
            format!("[{}]", count),
            Style::default().fg(theme::ACCENT).bg(theme::BADGE_BG),
        ),
    ]);
    let right = Line::from(vec![
        Span::styled(format!("{}s", secs), Style::default().fg(timer_col)),
    ]);

    f.render_widget(
        Paragraph::new(left).style(Style::default().bg(theme::SURFACE)),
        Rect { x: area.x, y: area.y, width: area.width / 2, height: 1 },
    );
    f.render_widget(
        Paragraph::new(right)
            .alignment(Alignment::Right)
            .style(Style::default().bg(theme::SURFACE)),
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

fn render_list(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault, secs: u64) {
    let timer_col = theme::timer_color(secs);
    let entries = vault.entries();

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let selected = i == state.selected_index;

            let label = if let Some(pos) = entry.name.find(':') {
                let issuer  = &entry.name[..pos];
                let account = &entry.name[pos + 1..];
                if account.is_empty() {
                    entry.name.clone()
                } else {
                    format!("{} · {}", issuer, account)
                }
            } else {
                entry.name.clone()
            };

            let show = state.show_codes || selected;
            let code_str = if show {
                let code = generate_code_now(&entry.secret)
                    .unwrap_or_else(|_| "000000".to_string());
                format!("{} {}", &code[..3], &code[3..])
            } else {
                "••• •••".to_string()
            };

            let (cursor, label_col, label_mod, code_col) = if selected {
                ("› ", theme::TEXT, Modifier::BOLD, timer_col)
            } else {
                (
                    "  ",
                    theme::DIM,
                    Modifier::empty(),
                    if state.show_codes { theme::DIM } else { theme::MUTED },
                )
            };

            // Expiry bar: 6 blocks representing seconds remaining in the 30s window
            const BAR_LEN: usize = 6;
            let bar_col = if selected { timer_col } else { theme::MUTED };
            let (expiry_bar, _) = if show {
                let filled = ((secs as usize * BAR_LEN) / 30).min(BAR_LEN);
                let bar = format!(
                    " {}{}",
                    "█".repeat(filled),
                    "░".repeat(BAR_LEN - filled),
                );
                let len = 1 + BAR_LEN; // space + bar chars (all single-width)
                (bar, len)
            } else {
                (String::new(), 0)
            };

            const GAP: &str = "  "; // fixed 2-space gap between label and code

            let line = Line::from(vec![
                Span::styled(cursor, Style::default().fg(theme::ACCENT)),
                Span::styled(label, Style::default().fg(label_col).add_modifier(label_mod)),
                Span::raw(GAP),
                Span::styled(code_str, Style::default().fg(code_col)),
                Span::styled(expiry_bar, Style::default().fg(bar_col)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().style(Style::default().bg(theme::BG)));

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
            "↑↓ nav  ⏎ détail  h codes  a add  d del  y copy  q quit",
            Style::default().fg(theme::MUTED),
        )))
        .style(Style::default().bg(theme::BG)),
        Rect { x: area.x, y: area.y + 1, width: area.width, height: 1 },
    );
}

fn render_toast(f: &mut Frame, area: Rect, state: &AppState) {
    let Some(msg) = &state.status_message else { return };

    let is_copy = msg.contains("Copied") || msg.contains("copied");
    let (border_col, text_col) = if is_copy {
        (theme::ACCENT, theme::ACCENT)
    } else {
        (theme::URGENT, theme::URGENT)
    };
    let label = if is_copy {
        format!("  ✓  {}  ", msg)
    } else {
        format!("  {}  ", msg)
    };
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
    let inner = Rect {
        x: toast.x + 1,
        y: toast.y + 1,
        width: toast.width.saturating_sub(2),
        height: 1,
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            label,
            Style::default().fg(text_col).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        inner,
    );
}
