use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::path::Path;

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault_path: &Path) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    // Vault path — bottom of screen, very discreet
    let raw_path = vault_path.to_string_lossy();
    let display_path = if let Some(home) = dirs::home_dir() {
        let home_str = home.to_string_lossy();
        if raw_path.starts_with(home_str.as_ref()) {
            format!("~{}", &raw_path[home_str.len()..])
        } else {
            raw_path.to_string()
        }
    } else {
        raw_path.to_string()
    };
    let max_path_w = (area.width as usize).saturating_sub(20);
    let shown_path = if display_path.len() > max_path_w {
        format!("…{}", &display_path[display_path.len() - max_path_w..])
    } else {
        display_path
    };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(shown_path, Style::default().fg(theme::MUTED)),
            Span::styled("  [ Tab ] copy", Style::default().fg(theme::MUTED)),
        ]))
        .alignment(Alignment::Center),
        Rect { x: area.x, y: area.y + area.height.saturating_sub(1), width: area.width, height: 1 },
    );

    // Centred modal content
    let content_w: u16 = area.width.min(54);
    let is_new = state.is_new_vault;

    let extra: u16 = if is_new { 3 } else { 0 } + if state.unlock_confirming { 4 } else { 0 };
    let content_h: u16 = 10 + extra;

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(content_h),
            Constraint::Fill(1),
        ])
        .split(area);

    let x = area.x + (area.width.saturating_sub(content_w)) / 2;
    let content = Rect { x, y: vert[1].y, width: content_w, height: content_h };

    let mut constraints = vec![
        Constraint::Length(1), // title
        Constraint::Length(1), // separator
        Constraint::Length(1), // gap
    ];
    if is_new {
        constraints.push(Constraint::Length(1)); // warning line 1
        constraints.push(Constraint::Length(1)); // warning line 2
        constraints.push(Constraint::Length(1)); // gap
    }
    constraints.push(Constraint::Length(1)); // passphrase label
    constraints.push(Constraint::Length(3)); // passphrase input
    if state.unlock_confirming {
        constraints.push(Constraint::Length(1)); // confirm label
        constraints.push(Constraint::Length(3)); // confirm input
    }
    constraints.push(Constraint::Length(1)); // error
    constraints.push(Constraint::Length(1)); // gap
    constraints.push(Constraint::Length(1)); // footer

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(content);

    let mut idx = 0;

    // Title
    let name = env!("CARGO_PKG_NAME");
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            name,
            Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[idx],
    );
    idx += 1;

    // Separator
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "─".repeat(name.len() + 4),
            Style::default().fg(theme::ACCENT),
        )))
        .alignment(Alignment::Center),
        rows[idx],
    );
    idx += 1;

    // Gap
    idx += 1;

    // Warning (new vault only)
    if is_new {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "⚠  Creating a new vault.",
                Style::default().fg(theme::URGENT).add_modifier(Modifier::BOLD),
            )))
            .alignment(Alignment::Center),
            rows[idx],
        );
        idx += 1;
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Remember your passphrase — it cannot be recovered.",
                Style::default().fg(theme::URGENT),
            )))
            .alignment(Alignment::Center),
            rows[idx],
        );
        idx += 1;
        idx += 1; // gap
    }

    // Passphrase label + input
    let pass_label = if is_new { "Choose a passphrase" } else { "Passphrase" };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(pass_label, Style::default().fg(theme::DIM)))),
        rows[idx],
    );
    idx += 1;

    let input_dots = "•".repeat(state.passphrase_input.len());
    let pass_active = !state.unlock_confirming;
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(input_dots, Style::default().fg(theme::DIM)),
            Span::styled(if pass_active { "▌" } else { "" }, Style::default().fg(theme::ACCENT)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(
                    if state.unlock_error && pass_active { theme::URGENT } else { theme::BORDER },
                ))
                .style(Style::default().bg(theme::BG)),
        ),
        rows[idx],
    );
    idx += 1;

    // Confirm passphrase (new vault, second step)
    if state.unlock_confirming {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled("Confirm passphrase", Style::default().fg(theme::DIM)))),
            rows[idx],
        );
        idx += 1;

        let confirm_dots = "•".repeat(state.passphrase_confirm.len());
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(confirm_dots, Style::default().fg(theme::DIM)),
                Span::styled("▌", Style::default().fg(theme::ACCENT)),
            ]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(
                        if state.unlock_error { theme::URGENT } else { theme::BORDER },
                    ))
                    .style(Style::default().bg(theme::BG)),
            ),
            rows[idx],
        );
        idx += 1;
    }

    // Error message
    if let Some(msg) = &state.unlock_error_msg {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                msg.as_str(),
                Style::default().fg(theme::URGENT),
            )))
            .alignment(Alignment::Center),
            rows[idx],
        );
    } else if state.unlock_error {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Wrong passphrase.",
                Style::default().fg(theme::URGENT),
            )))
            .alignment(Alignment::Center),
            rows[idx],
        );
    }
    idx += 1;

    // Gap
    idx += 1;

    // Footer
    let footer = if state.unlock_confirming {
        "⏎ confirm · Esc back · ^C quit"
    } else if is_new {
        "⏎ continue · ^C quit"
    } else {
        "⏎ unlock · ^C quit"
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(footer, Style::default().fg(theme::MUTED))))
            .alignment(Alignment::Center),
        rows[idx],
    );
}
