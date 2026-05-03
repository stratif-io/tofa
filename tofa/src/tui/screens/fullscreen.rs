use crate::tui::state::AppState;
use tofa_theme::palette as theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph},
    Frame,
};
use tofa_core::{
    store::Vault,
    totp::{format_code, generate_code_now, seconds_remaining_now},
};
use tui_big_text::{BigText, PixelSize};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let entry = match vault.entries().get(state.selected_index) {
        Some(e) => e,
        None => return,
    };

    let code = generate_code_now(entry).unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now(entry);
    let timer_col = tofa_theme::palette::timer_color(secs);

    let is_8digit = entry.digits == 8;
    let use_half_height = if is_8digit { area.width >= 80 } else { area.width >= 68 };
    let (modal_w, modal_h) = if use_half_height {
        let min_w = if is_8digit { 82u16 } else { 70u16 };
        (area.width.min(min_w), 13u16)
    } else {
        let min_w = if is_8digit { 46u16 } else { 38u16 };
        (area.width.min(min_w).max(min_w.min(area.width)), 13u16)
    };
    let modal_x = area.x + (area.width.saturating_sub(modal_w)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_h)) / 2;
    let modal = Rect { x: modal_x, y: modal_y, width: modal_w, height: modal_h };

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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // name
            Constraint::Length(1), // gap
            Constraint::Length(4), // big text
            Constraint::Length(1), // gap
            Constraint::Length(1), // gauge
            Constraint::Length(1), // timer text
            Constraint::Fill(1),   // spacer
            Constraint::Length(1), // help
        ])
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
    let pixel_size = if use_half_height { PixelSize::HalfHeight } else { PixelSize::Quadrant };
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
        .split(chunks[4]);
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
        chunks[5],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ y ] copy   [ Esc ] back",
            Style::default().fg(theme::TEXT_MUTED),
        )))
        .alignment(Alignment::Center),
        chunks[7],
    );

    super::toast::render(f, area, state);
}
