use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rotp_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let entry = match vault.entries().get(state.selected_index) {
        Some(e) => e,
        None => return,
    };

    let code = generate_code_now(&entry.secret).unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);

    let rows: Vec<(&str, String, bool)> = vec![
        ("Name",    entry.name.clone(),    false),
        ("Secret",  entry.secret.clone(),  false),
        ("Code",    format!("{} {}  {}s", &code[..3], &code[3..], secs), true),
        ("Created", entry.created_at.clone(), false),
    ];

    let box_h = (rows.len() as u16 + 6).min(area.height.saturating_sub(4));
    let box_w = area.width.min(62);
    let pad_x = (area.width.saturating_sub(box_w)) / 2;
    let pad_y = (area.height.saturating_sub(box_h)) / 2;
    let modal = Rect { x: area.x + pad_x, y: area.y + pad_y, width: box_w, height: box_h };

    f.render_widget(Clear, modal);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER))
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
            "OTP Details",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        ))),
        chunks[0],
    );

    let mut content: Vec<Line> = vec![Line::from("")];
    for (label, value, is_code) in &rows {
        content.push(Line::from(vec![
            Span::styled(format!("{label:<10}"), Style::default().fg(theme::DIM)),
            Span::styled(
                value.clone(),
                Style::default()
                    .fg(if *is_code { timer_col } else { theme::TEXT })
                    .add_modifier(if *is_code { Modifier::BOLD } else { Modifier::empty() }),
            ),
        ]));
    }
    f.render_widget(Paragraph::new(content), chunks[2]);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ y ] copy code   [ Esc ] back",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Center),
        chunks[3],
    );
}
