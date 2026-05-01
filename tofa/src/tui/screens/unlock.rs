use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    // Centre a column of content vertically and horizontally
    let content_w: u16 = 28;
    // title(1) + sep(1) + gap(1) + label(1) + input(1) + error(1) + gap(1) + footer(1) = 8
    let content_h: u16 = 8;

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

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(1), // separator
            Constraint::Length(1), // gap
            Constraint::Length(1), // label
            Constraint::Length(1), // input box
            Constraint::Length(1), // error
            Constraint::Length(1), // gap
            Constraint::Length(1), // footer
        ])
        .split(content);

    // Title — large, bold, no letter-spacing
    let name = env!("CARGO_PKG_NAME");
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            name,
            Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[0],
    );

    // Thin separator line in ACCENT
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "─".repeat(content_w as usize / 3),
            Style::default().fg(theme::ACCENT),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );

    // "Passphrase" label
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Passphrase",
            Style::default().fg(theme::DIM),
        ))),
        rows[3],
    );

    // Input box with border
    let input_dots = "•".repeat(state.passphrase_input.len());
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(input_dots, Style::default().fg(theme::DIM)),
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
        rows[4],
    );

    // Error message
    if state.unlock_error {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Wrong passphrase.",
                Style::default().fg(theme::URGENT),
            )))
            .alignment(Alignment::Center),
            rows[5],
        );
    }

    // Footer
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "⏎ unlock · ^C quit",
            Style::default().fg(theme::MUTED),
        )))
        .alignment(Alignment::Center),
        rows[7],
    );
}
