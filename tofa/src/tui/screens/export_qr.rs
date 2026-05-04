use crate::theme::palette as theme;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let qr_h = state.export_qr_lines.len() as u16;
    let qr_w = state
        .export_qr_lines
        .first()
        .map(|l| l.chars().count() as u16)
        .unwrap_or(0);

    // Required terminal size: QR + border (2) + title (1) + blank (1) + help (1)
    let needed_w = qr_w + 4;
    let needed_h = qr_h + 4;

    if area.width < needed_w || area.height < needed_h {
        render_too_small(f, area, needed_w, needed_h);
        return;
    }

    let box_w = needed_w.min(area.width);
    let box_h = needed_h.min(area.height);
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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Scan with your authenticator app",
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        chunks[0],
    );

    let qr_rect = Rect {
        x: chunks[2].x + (chunks[2].width.saturating_sub(qr_w)) / 2,
        y: chunks[2].y + (chunks[2].height.saturating_sub(qr_h)) / 2,
        width: qr_w,
        height: qr_h,
    };

    let qr_style = Style::default().fg(Color::Black).bg(Color::White);
    let qr_lines: Vec<Line> = state
        .export_qr_lines
        .iter()
        .map(|row| Line::from(Span::styled(row.clone(), qr_style)))
        .collect();

    f.render_widget(Clear, qr_rect);
    f.render_widget(
        Block::default().style(Style::default().bg(Color::White)),
        qr_rect,
    );
    f.render_widget(Paragraph::new(qr_lines), qr_rect);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ Esc ] back",
            Style::default().fg(theme::TEXT_MUTED),
        )))
        .alignment(Alignment::Center),
        chunks[3],
    );
}

fn render_too_small(f: &mut Frame, area: Rect, needed_w: u16, needed_h: u16) {
    let box_h = 7u16;
    let box_w = area.width.min(52);
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
            .border_style(Style::default().fg(theme::DANGER))
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
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Terminal too small",
            Style::default()
                .fg(theme::DANGER)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("Need at least {}×{} — resize and retry", needed_w, needed_h),
            Style::default().fg(theme::TEXT_MUTED),
        )))
        .alignment(Alignment::Center),
        rows[2],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ Esc ] back",
            Style::default().fg(theme::TEXT_MUTED),
        )))
        .alignment(Alignment::Center),
        rows[3],
    );
}
