use crate::theme::palette as theme;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use tofa_core::{
    store::Vault,
    totp::{format_code, generate_code_now, seconds_remaining_now},
};

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

    // The full otpauth:// URI for this entry. The secret is masked by
    // default and revealed alongside the secret field when the user
    // unlocks via `s` — same passphrase prompt, same trust boundary.
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

    // Extra rows when the passphrase prompt is active
    let extra_rows: u16 = if state.detail_revealing { 3 } else { 0 };
    // Modal needs to fit (with borders): 1 title + 1 gap + 5 fields +
    // 3 rows for the wrapped URI + 1 help = 11 inner, +2 borders = 13.
    let box_h = (13 + extra_rows).min(area.height.saturating_sub(4));
    // Wide enough that "URI: otpauth://totp/Issuer:account?secret=..."
    // fits on at most three lines for typical entries.
    let box_w = area.width.min(78);
    let pad_x = (area.width.saturating_sub(box_w)) / 2;
    let pad_y = (area.height.saturating_sub(box_h)) / 2;
    let modal = Rect {
        x: area.x + pad_x,
        y: area.y + pad_y,
        width: box_w,
        height: box_h,
    };

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

    let algo_str = format!(
        "{} · {}d · {}s",
        entry.algorithm, entry.digits, entry.period
    );

    // Title row + field rows + URI row + optional reveal prompt + help row
    let mut constraints = vec![
        Constraint::Length(1), // title
        Constraint::Length(1), // gap
        Constraint::Length(1), // name
        Constraint::Length(1), // code
        Constraint::Length(1), // algorithm · digits · period
        Constraint::Length(1), // secret
        Constraint::Length(1), // created
        Constraint::Length(3), // URI (wraps to up to 3 lines for ~150-char URIs)
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

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "OTP Details",
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        ))),
        chunks[0],
    );

    let field_rows: &[(&str, &str, bool)] = &[
        ("Name", entry.name.as_str(), false),
        ("Code", &format!("{}  {}s", format_code(&code), secs), true),
        ("Params", &algo_str, false),
        ("Secret", &secret_display, false),
        ("Created", entry.created_at.as_str(), false),
    ];

    for (i, (label, value, is_code)) in field_rows.iter().enumerate() {
        let value_col = if *is_code { timer_col } else { theme::TEXT };
        let value_mod = if *is_code {
            Modifier::BOLD
        } else {
            Modifier::empty()
        };
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{label:<10}"),
                    Style::default().fg(theme::TEXT_MUTED),
                ),
                Span::styled(
                    *value,
                    Style::default().fg(value_col).add_modifier(value_mod),
                ),
            ])),
            chunks[2 + i],
        );
    }

    // URI row — its own paragraph so we can wrap onto a second line
    // when the URI exceeds the modal width (almost always).
    use ratatui::widgets::Wrap;
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<10}", "URI"),
                Style::default().fg(theme::TEXT_MUTED),
            ),
            Span::styled(uri_display.as_str(), Style::default().fg(theme::TEXT)),
        ]))
        .wrap(Wrap { trim: false }),
        chunks[7],
    );

    let help_idx = if state.detail_revealing {
        // render passphrase prompt rows
        let gap_idx = 8;
        let label_idx = 9;
        let input_idx = 10;
        let help_idx = 11;

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
        8
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
