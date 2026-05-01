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

    let box_height = 10u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(52);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let outer = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::BG)),
        outer,
    );

    let inner = Rect {
        x: outer.x + 1,
        y: outer.y + 1,
        width: outer.width.saturating_sub(2),
        height: outer.height.saturating_sub(2),
    };

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(1), // subtitle
            Constraint::Length(1), // gap
            Constraint::Length(1), // label
            Constraint::Length(1), // input
            Constraint::Length(1), // error
            Constraint::Length(1), // gap
            Constraint::Length(1), // help
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            &env!("CARGO_PKG_NAME").chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" "),
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "OTP Manager",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Passphrase",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Left),
        rows[3],
    );

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "•".repeat(state.passphrase_input.len()),
                Style::default().fg(theme::DIM),
            ),
            Span::styled("▌", Style::default().fg(theme::ACCENT)),
        ]))
        .alignment(Alignment::Left),
        rows[4],
    );

    if state.unlock_error {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Wrong passphrase. Try again.",
                Style::default().fg(theme::URGENT),
            )))
            .alignment(Alignment::Center),
            rows[5],
        );
    }

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ Enter ] unlock   [ Ctrl+C ] quit",
            Style::default().fg(theme::MUTED),
        )))
        .alignment(Alignment::Center),
        rows[7],
    );
}
