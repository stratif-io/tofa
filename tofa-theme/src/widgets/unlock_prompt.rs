use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use crate::{palette, theme::ThemeMode, wink};

pub struct UnlockPrompt {
    pub masked_input: String,
    pub error:        Option<String>,
    pub theme:        ThemeMode,
}

impl UnlockPrompt {
    pub fn new() -> Self { Self { masked_input: String::new(), error: None, theme: ThemeMode::Dark } }
    pub fn input(mut self, s: impl Into<String>) -> Self { self.masked_input = s.into(); self }
    pub fn error(mut self, e: impl Into<String>) -> Self { self.error = Some(e.into()); self }
    pub fn theme(mut self, t: ThemeMode) -> Self { self.theme = t; self }
}

impl Widget for UnlockPrompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let brand = self.theme.brand();
        let muted = self.theme.text_muted();

        let total_h = wink::WINK_LARGE_HEIGHT + 4;
        let top = area.y + area.height.saturating_sub(total_h) / 2;

        // Sir Wink — centered
        for (i, line) in wink::WINK_LARGE.lines().enumerate() {
            let y = top + i as u16;
            if y >= area.y + area.height { break; }
            let x = area.x + area.width.saturating_sub(wink::WINK_LARGE_WIDTH) / 2;
            let wink_area = Rect::new(x, y, wink::WINK_LARGE_WIDTH, 1);
            Paragraph::new(Line::from(Span::styled(line, Style::default().fg(brand))))
                .render(wink_area, buf);
        }

        let label_y = top + wink::WINK_LARGE_HEIGHT;
        if label_y < area.y + area.height {
            let label = "VAULT LOCKED";
            let x = area.x + area.width.saturating_sub(label.len() as u16) / 2;
            let label_area = Rect::new(x, label_y, label.len() as u16, 1);
            Paragraph::new(Line::from(Span::styled(label, Style::default().fg(muted))))
                .render(label_area, buf);
        }

        let input_y = label_y + 2;
        if input_y < area.y + area.height {
            let dots: String = "●".repeat(self.masked_input.len().min(20));
            let display = format!(" {} ", if dots.is_empty() { "passphrase…".to_string() } else { dots });
            let w = (display.len() as u16 + 2).min(area.width);
            let x = area.x + area.width.saturating_sub(w) / 2;
            let input_area = Rect::new(x, input_y, w, 1);
            Paragraph::new(Line::from(Span::styled(display, Style::default().fg(self.theme.text()).bg(self.theme.surface()))))
                .block(Block::default().style(Style::default().fg(brand)))
                .render(input_area, buf);
        }

        if let Some(err) = self.error {
            let err_y = input_y + 1;
            if err_y < area.y + area.height {
                let x = area.x + area.width.saturating_sub(err.len() as u16) / 2;
                let err_area = Rect::new(x, err_y, err.len() as u16, 1);
                Paragraph::new(Line::from(Span::styled(err, Style::default().fg(palette::DANGER))))
                    .render(err_area, buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn renders_vault_locked_label() {
        let backend = TestBackend::new(40, 20);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| {
            f.render_widget(UnlockPrompt::new(), f.area());
        }).unwrap();
        let content: String = terminal.backend().buffer().content().iter().map(|c| c.symbol().to_string()).collect();
        assert!(content.contains("VAULT LOCKED"));
    }
}
