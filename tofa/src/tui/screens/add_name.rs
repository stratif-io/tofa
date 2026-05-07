use crate::theme::palette as theme;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use tofa_core::{
    qr::OtpSecret,
    totp::{format_code, generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    // Build a temporary entry through the shared core helper so the
    // SHA1 / 6 / 30 defaults match exactly what the AddEntry handler
    // will write to the vault. Empty name + created_at — they're not
    // used by generate_code_now / seconds_remaining_now.
    let tmp_entry = OtpSecret {
        secret: state.add_parsed_secret.as_str().to_string(),
        meta: state.add_meta.clone().unwrap_or_default(),
    }
    .into_vault_entry(String::new(), String::new());
    let code = generate_code_now(&tmp_entry).unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now(&tmp_entry);
    let timer_col = crate::theme::palette::timer_color(secs);

    let error_line = if let Some(msg) = &state.status_message {
        Line::from(Span::styled(
            msg.as_str(),
            Style::default().fg(theme::DANGER),
        ))
    } else {
        Line::from("")
    };

    let mut meta_lines: Vec<Line> = Vec::new();
    if let Some(meta) = &state.add_meta {
        if let Some(issuer) = &meta.issuer {
            meta_lines.push(Line::from(vec![
                Span::styled("Issuer:    ", Style::default().fg(theme::TEXT_MUTED)),
                Span::styled(issuer.clone(), Style::default().fg(theme::TEXT)),
            ]));
        }
        if let Some(account) = &meta.account {
            meta_lines.push(Line::from(vec![
                Span::styled("Account:   ", Style::default().fg(theme::TEXT_MUTED)),
                Span::styled(account.clone(), Style::default().fg(theme::TEXT)),
            ]));
        }
        // Reuse the resolved values from tmp_entry — same defaults
        // as the actual save path, no second source of truth.
        meta_lines.push(Line::from(vec![
            Span::styled("Algorithm: ", Style::default().fg(theme::TEXT_MUTED)),
            Span::styled(
                format!(
                    "{}  {} digits  {}s",
                    tmp_entry.algorithm, tmp_entry.digits, tmp_entry.period
                ),
                Style::default().fg(theme::TEXT_MUTED),
            ),
        ]));
    }

    let box_height = (10 + meta_lines.len()).min(20) as u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(60);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let inner = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    let mut content = vec![
        Line::from(Span::styled(
            "Add OTP",
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled(
                format!("{}  ", format_code(&code)),
                Style::default()
                    .fg(theme::BRAND)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(format!("{}s", secs), Style::default().fg(timer_col)),
        ]),
        Line::from(""),
    ];

    content.extend(meta_lines);
    if state.add_meta.is_some() {
        content.push(Line::from(""));
    }
    content.push(Line::from(Span::styled(
        "Name for this account:",
        Style::default().fg(theme::TEXT_MUTED),
    )));
    content.push(Line::from(Span::styled(
        format!("{}_", state.add_name),
        Style::default().fg(theme::TEXT),
    )));
    content.push(Line::from(""));
    content.push(error_line);
    content.push(Line::from(""));
    content.push(Line::from(Span::styled(
        "[ Enter ] save   [ Esc ] back",
        Style::default().fg(theme::TEXT_MUTED),
    )));

    f.render_widget(
        Paragraph::new(content).alignment(Alignment::Left).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::BORDER))
                .style(Style::default().bg(theme::BG)),
        ),
        inner,
    );
}
