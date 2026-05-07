use crate::theme::palette as theme;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph, Wrap},
    Frame,
};
use tofa_core::{
    store::Vault,
    totp::{format_code, generate_code_now, seconds_remaining_now},
};
use tui_big_text::{BigText, PixelSize};

/// Single full-screen view for an entry. Big readable code at the top,
/// metadata + secret + URI rows below — collapses what used to be the
/// separate Fullscreen and Info modals into one screen the user can
/// reach via Space (from the list) or `i`.
pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let entry = match vault.entries().get(state.selected_index) {
        Some(e) => e,
        None => return,
    };

    let code = generate_code_now(entry).unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now(entry);
    let timer_col = crate::theme::palette::timer_color(secs);

    let secret_display = if state.detail_secret_visible {
        entry.secret.clone()
    } else {
        "•".repeat(entry.secret.len().max(16))
    };
    let full_uri = tofa_core::qr::build_otpauth_uri(entry);
    let uri_display: String = if state.detail_secret_visible {
        full_uri.clone()
    } else {
        full_uri.replacen(
            entry.secret.as_str(),
            &"•".repeat(entry.secret.len().max(16)),
            1,
        )
    };

    let is_8digit = entry.digits == 8;
    let use_half_height = if is_8digit {
        area.width >= 80
    } else {
        area.width >= 68
    };
    let modal_w = if use_half_height {
        let min_w = if is_8digit { 82u16 } else { 78u16 };
        area.width.min(min_w)
    } else {
        let min_w = if is_8digit { 60u16 } else { 60u16 };
        area.width.min(min_w).max(min_w.min(area.width))
    };
    // Height: name(1) + gap(1) + big(4) + gauge(1) + timer(1) + gap(1)
    //       + 4 detail rows + URI(3 wrapped lines) + (reveal extras) + help(1)
    //       + 2 borders. Reveal mode adds 3 rows (gap, label, input).
    let extra_rows: u16 = if state.detail_revealing { 3 } else { 0 };
    let modal_h = (20 + extra_rows).min(area.height.saturating_sub(2));
    let modal_x = area.x + (area.width.saturating_sub(modal_w)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_h)) / 2;
    let modal = Rect {
        x: modal_x,
        y: modal_y,
        width: modal_w,
        height: modal_h,
    };

    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);
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

    let mut constraints = vec![
        Constraint::Length(1), // 0 name
        Constraint::Length(1), // 1 gap
        Constraint::Length(4), // 2 big code
        Constraint::Length(1), // 3 gauge
        Constraint::Length(1), // 4 timer text
        Constraint::Length(1), // 5 gap
        Constraint::Length(1), // 6 Params
        Constraint::Length(1), // 7 Secret
        Constraint::Length(1), // 8 Created
        Constraint::Length(3), // 9 URI (wraps to up to 3 lines)
    ];
    if state.detail_revealing {
        constraints.push(Constraint::Length(1)); // gap
        constraints.push(Constraint::Length(1)); // passphrase label
        constraints.push(Constraint::Length(1)); // passphrase input
    }
    constraints.push(Constraint::Length(1)); // help

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    let header = if let Some(pos) = entry.name.find(':') {
        format!("{} · {}", &entry.name[..pos], &entry.name[pos + 1..])
    } else {
        entry.name.clone()
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            header,
            Style::default().fg(theme::TEXT_MUTED),
        )))
        .alignment(Alignment::Center),
        chunks[0],
    );

    let spaced = format_code(&code);
    let pixel_size = if use_half_height {
        PixelSize::HalfHeight
    } else {
        PixelSize::Quadrant
    };
    let big_text = BigText::builder()
        .pixel_size(pixel_size)
        .style(Style::default().fg(timer_col).add_modifier(Modifier::BOLD))
        .lines(vec![Line::from(spaced.as_str())])
        .centered()
        .build();
    f.render_widget(big_text, chunks[2]);

    let ratio = secs as f64 / entry.period as f64;
    let gauge_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(chunks[3]);
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(Style::default().fg(timer_col).bg(theme::BG))
        .label("")
        .ratio(ratio);
    f.render_widget(gauge, gauge_area[1]);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("expires in {}s", secs),
            Style::default().fg(timer_col),
        )))
        .alignment(Alignment::Center),
        chunks[4],
    );

    let algo_str = format!(
        "{} · {}d · {}s",
        entry.algorithm, entry.digits, entry.period
    );
    let detail_rows: &[(&str, &str)] = &[
        ("Params", &algo_str),
        ("Secret", &secret_display),
        ("Created", entry.created_at.as_str()),
    ];
    for (i, (label, value)) in detail_rows.iter().enumerate() {
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{label:<10}"),
                    Style::default().fg(theme::TEXT_MUTED),
                ),
                Span::styled(*value, Style::default().fg(theme::TEXT)),
            ])),
            chunks[6 + i],
        );
    }

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<10}", "URI"),
                Style::default().fg(theme::TEXT_MUTED),
            ),
            Span::styled(uri_display.as_str(), Style::default().fg(theme::TEXT)),
        ]))
        .wrap(Wrap { trim: false }),
        chunks[9],
    );

    let help_idx = if state.detail_revealing {
        let gap_idx = 10;
        let label_idx = 11;
        let input_idx = 12;
        let help_idx = 13;

        f.render_widget(Paragraph::new(Line::from("")), chunks[gap_idx]);
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Passphrase to reveal secret:",
                Style::default().fg(theme::TEXT_MUTED),
            ))),
            chunks[label_idx],
        );
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    "•".repeat(state.detail_passphrase.len()),
                    Style::default().fg(theme::TEXT_MUTED),
                ),
                Span::styled("▌", Style::default().fg(theme::BRAND)),
            ])),
            chunks[input_idx],
        );

        help_idx
    } else {
        10
    };

    let key = |k: &'static str| Span::styled(k, Style::default().fg(theme::BRAND));
    let desc = |d: &'static str| Span::styled(d, Style::default().fg(theme::TEXT));
    let sep = || Span::styled("  ", Style::default());
    let reveal_label = if state.detail_secret_visible {
        " hide secret"
    } else {
        " reveal secret"
    };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            key("y"),
            desc(" copy code"),
            sep(),
            key("u"),
            desc(" copy URI"),
            sep(),
            key("s"),
            desc(reveal_label),
            sep(),
            key("esc"),
            desc(" back"),
        ]))
        .alignment(Alignment::Center),
        chunks[help_idx],
    );

    super::toast::render(f, area, state);
}
