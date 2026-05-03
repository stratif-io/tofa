use crate::tui::state::AppState;
use tofa_theme::palette as theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let Some(msg) = &state.status_message else { return };

    let is_copy = msg.contains("Copied") || msg.contains("copied");
    let (border_col, text_col) = if is_copy {
        (theme::BRAND, theme::BRAND)
    } else {
        (theme::DANGER, theme::DANGER)
    };
    let label = if is_copy {
        format!("  ✓  {}  ", msg)
    } else {
        format!("  {}  ", msg)
    };
    let toast_w = (label.chars().count() as u16 + 2).min(area.width);
    let toast_h = 3u16;
    let toast = Rect {
        x: area.x + (area.width.saturating_sub(toast_w)) / 2,
        y: area.y + (area.height.saturating_sub(toast_h)) / 2,
        width: toast_w,
        height: toast_h,
    };
    f.render_widget(Clear, toast);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_col))
            .style(Style::default().bg(theme::BG)),
        toast,
    );
    let inner = Rect {
        x: toast.x + 1,
        y: toast.y + 1,
        width: toast.width.saturating_sub(2),
        height: 1,
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            label,
            Style::default().fg(text_col).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        inner,
    );
}
