use crate::theme::palette as theme;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Plain-text view of the otpauth:// URIs for the user's checked
/// selection. Mouse capture is disabled by `run_app` while this screen
/// is active, so the user can click-and-drag to select any URI text
/// natively. `y` copies the whole list to the clipboard.
pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let modal_w = area.width.min(100);
    let modal_h = area.height.saturating_sub(2);
    let modal_x = area.x + (area.width.saturating_sub(modal_w)) / 2;
    let modal_y = area.y + 1;
    let modal = Rect {
        x: modal_x,
        y: modal_y,
        width: modal_w,
        height: modal_h,
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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(1), // gap
            Constraint::Min(1),    // body (scrollable)
            Constraint::Length(1), // help
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Export — URIs",
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        ))),
        chunks[0],
    );

    let mut lines: Vec<Line> = Vec::new();
    for (i, (name, uri)) in state.export_uri_list.iter().enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(
            name.clone(),
            Style::default().fg(theme::TEXT_MUTED),
        )));
        lines.push(Line::from(Span::styled(
            uri.clone(),
            Style::default().fg(theme::TEXT),
        )));
    }

    f.render_widget(
        Paragraph::new(lines)
            .scroll((state.export_uri_scroll, 0))
            .wrap(Wrap { trim: false }),
        chunks[2],
    );

    let key = |k: &'static str| Span::styled(k, Style::default().fg(theme::BRAND));
    let desc = |d: &'static str| Span::styled(d, Style::default().fg(theme::TEXT));
    let sep = || Span::styled("  ", Style::default());
    f.render_widget(
        Paragraph::new(Line::from(vec![
            key("y"),
            desc(" copy all"),
            sep(),
            key("↑↓"),
            desc(" scroll"),
            sep(),
            key("esc"),
            desc(" back"),
        ]))
        .alignment(Alignment::Center),
        chunks[3],
    );

    super::toast::render(f, area, state);
}
