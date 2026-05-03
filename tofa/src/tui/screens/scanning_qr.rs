use crate::tui::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use tofa_theme::palette as theme;

pub fn render(f: &mut Frame, area: Rect, _state: &AppState) {
    let box_h = 5u16;
    let box_w = area.width.min(36);
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
            .border_style(Style::default().fg(theme::BRAND))
            .style(Style::default().bg(theme::BG)),
        modal,
    );

    let inner = Rect {
        x: modal.x + 1,
        y: modal.y + 1,
        width: modal.width.saturating_sub(2),
        height: modal.height.saturating_sub(2),
    };

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "⟳ ",
                Style::default()
                    .fg(theme::BRAND)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Analyzing QR code…", Style::default().fg(theme::TEXT_MUTED)),
        ]))
        .alignment(Alignment::Center),
        rows[1],
    );
}
