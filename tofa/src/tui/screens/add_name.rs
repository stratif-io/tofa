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
    store::VaultEntry,
    totp::{format_code, generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    // Build a temporary entry to generate a code with the correct params
    let tmp_entry = {
        let (period, digits, algorithm) = state
            .add_meta
            .as_ref()
            .map(|m| {
                (
                    m.period.unwrap_or(30),
                    m.digits.unwrap_or(6),
                    m.algorithm.clone().unwrap_or_else(|| "SHA1".to_string()),
                )
            })
            .unwrap_or((30, 6, "SHA1".to_string()));
        VaultEntry {
            id: String::new(),
            name: String::new(),
            secret: state.add_parsed_secret.as_str().to_string(),
            created_at: String::new(),
            period,
            digits,
            algorithm,
        }
    };
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
        let algo = meta.algorithm.as_deref().unwrap_or("SHA1");
        let digits = meta.digits.unwrap_or(6);
        let period = meta.period.unwrap_or(30);
        meta_lines.push(Line::from(vec![
            Span::styled("Algorithm: ", Style::default().fg(theme::TEXT_MUTED)),
            Span::styled(
                format!("{algo}  {digits} digits  {period}s"),
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
