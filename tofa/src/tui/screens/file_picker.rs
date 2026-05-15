use crate::theme::palette as theme;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

#[must_use]
pub fn filtered<'a>(entries: &'a [(String, bool)], query: &str) -> Vec<&'a (String, bool)> {
    let q = query.to_lowercase();
    entries
        .iter()
        .filter(|(name, _)| q.is_empty() || name.to_lowercase().contains(&q))
        .collect()
}

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let box_h = area.height.saturating_sub(4).max(8);
    let box_w = area.width.min(72);
    let pad_x = (area.width.saturating_sub(box_w)) / 2;
    let pad_y = (area.height.saturating_sub(box_h)) / 2;
    let modal = Rect {
        x: area.x + pad_x,
        y: area.y + pad_y,
        width: box_w,
        height: box_h,
    };

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
            Constraint::Length(1), // path
            Constraint::Length(1), // search
            Constraint::Length(1), // gap
            Constraint::Min(1),    // list
            Constraint::Length(1), // help
        ])
        .split(inner);

    let path_str = state.fp_path.to_string_lossy();
    let max_w = chunks[0].width as usize;
    let truncated = if path_str.len() > max_w {
        format!("…{}", &path_str[path_str.len() - max_w + 1..])
    } else {
        path_str.to_string()
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            truncated,
            Style::default().fg(theme::TEXT_MUTED),
        ))),
        chunks[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("/ ", Style::default().fg(theme::TEXT_MUTED)),
            Span::styled(
                state.fp_query.clone(),
                Style::default()
                    .fg(theme::TEXT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("▌", Style::default().fg(theme::BRAND)),
        ])),
        chunks[1],
    );

    let visible: Vec<&(String, bool)> = filtered(&state.fp_entries, &state.fp_query);
    let items: Vec<ListItem> = visible
        .iter()
        .enumerate()
        .map(|(i, (name, is_dir))| {
            let selected = i == state.fp_selected;
            let border_col = if selected { theme::BRAND } else { theme::BG };
            let text_col = if *is_dir {
                theme::TEXT_MUTED
            } else {
                theme::TEXT
            };
            // Files: 3-cell checkbox slot ("[✓]"/"[ ]"). Directories
            // get a 3-cell directional arrow so the indent lines up
            // visually whether or not the row is checkable.
            let leading = if *is_dir {
                Span::styled("▸  ", Style::default().fg(text_col))
            } else {
                let full = state.fp_path.join(name);
                let checked = state.fp_checked.iter().any(|p| p == &full);
                if checked {
                    Span::styled(
                        "[✓]",
                        Style::default()
                            .fg(theme::BRAND)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled("[ ]", Style::default().fg(theme::TEXT_MUTED))
                }
            };
            ListItem::new(Line::from(vec![
                Span::styled("▌ ", Style::default().fg(border_col)),
                leading,
                Span::styled(" ", Style::default()),
                Span::styled(name.clone(), Style::default().fg(text_col)),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.fp_selected));
    f.render_stateful_widget(
        List::new(items)
            .block(Block::default().style(Style::default().bg(theme::BG)))
            .highlight_style(Style::default().bg(theme::SURFACE)),
        chunks[3],
        &mut list_state,
    );

    let key = |k: &'static str| Span::styled(k, Style::default().fg(theme::BRAND));
    let desc = |d: &'static str| Span::styled(d, Style::default().fg(theme::TEXT));
    let sep = || Span::styled("  ", Style::default());
    let mut hint = vec![
        key("↑↓"),
        desc(" nav"),
        sep(),
        key("spc"),
        desc(" toggle"),
        sep(),
        key("⏎"),
        desc(" open/import"),
        sep(),
        key("esc"),
        desc(" back"),
    ];
    let count = state.fp_checked.len();
    if count > 0 {
        hint.push(sep());
        hint.push(Span::styled(
            format!("({count} selected)"),
            Style::default()
                .fg(theme::BRAND)
                .add_modifier(Modifier::BOLD),
        ));
    }
    f.render_widget(Paragraph::new(Line::from(hint)), chunks[4]);
}
