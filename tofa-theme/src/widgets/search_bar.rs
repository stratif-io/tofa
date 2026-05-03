use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::theme::ThemeMode;

pub struct SearchBar {
    pub query:   String,
    pub focused: bool,
    pub theme:   ThemeMode,
}

impl SearchBar {
    pub fn new(query: impl Into<String>) -> Self {
        Self { query: query.into(), focused: false, theme: ThemeMode::Dark }
    }
    pub fn focused(mut self, f: bool) -> Self { self.focused = f; self }
    pub fn theme(mut self, t: ThemeMode) -> Self { self.theme = t; self }
}

impl Widget for SearchBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let brand = self.theme.brand();
        let border_color = if self.focused { brand } else { self.theme.border() };
        let display = if self.query.is_empty() {
            "/ search…".to_string()
        } else {
            format!("/ {}", self.query)
        };
        let text_color = if self.query.is_empty() { self.theme.text_muted() } else { self.theme.text() };
        Paragraph::new(Line::from(Span::styled(display, Style::default().fg(text_color))))
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)))
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn empty_query_shows_placeholder() {
        let backend = TestBackend::new(20, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| {
            f.render_widget(SearchBar::new(""), f.area());
        }).unwrap();
        let content: String = terminal.backend().buffer().content().iter().map(|c| c.symbol().to_string()).collect();
        assert!(content.contains("search"));
    }

    #[test]
    fn non_empty_query_shows_slash_prefix() {
        let backend = TestBackend::new(20, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| {
            f.render_widget(SearchBar::new("git"), f.area());
        }).unwrap();
        let content: String = terminal.backend().buffer().content().iter().map(|c| c.symbol().to_string()).collect();
        assert!(content.contains("/"));
        assert!(content.contains("git"));
    }
}
