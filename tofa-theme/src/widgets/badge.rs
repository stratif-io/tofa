use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::{Line, Span}, widgets::Widget};
use crate::theme::ThemeMode;

pub enum BadgeVariant { Success, Warning, Danger, Brand }

pub struct Badge {
    pub text:    String,
    pub variant: BadgeVariant,
    pub theme:   ThemeMode,
}

impl Badge {
    pub fn new(text: impl Into<String>, variant: BadgeVariant) -> Self {
        Self { text: text.into(), variant, theme: ThemeMode::Dark }
    }
    pub fn theme(mut self, theme: ThemeMode) -> Self { self.theme = theme; self }
}

impl Widget for Badge {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use crate::palette;
        let color = match self.variant {
            BadgeVariant::Success => palette::SUCCESS,
            BadgeVariant::Warning => palette::WARNING,
            BadgeVariant::Danger  => palette::DANGER,
            BadgeVariant::Brand   => self.theme.brand(),
        };
        let label = format!("● {}", self.text);
        let x = area.x;
        let y = area.y;
        for (i, ch) in label.chars().enumerate() {
            if x + i as u16 >= area.x + area.width { break; }
            buf[(x + i as u16, y)]
                .set_char(ch)
                .set_style(Style::default().fg(color));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    fn render_badge(badge: Badge, width: u16) -> ratatui::buffer::Buffer {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| {
            f.render_widget(badge, f.area());
        }).unwrap();
        terminal.backend().buffer().clone()
    }

    #[test]
    fn badge_renders_dot_and_text() {
        let badge = Badge::new("Synced", BadgeVariant::Success);
        let buf = render_badge(badge, 12);
        let content: String = buf.content().iter().map(|c| c.symbol().to_string()).collect();
        assert!(content.contains("●"));
        assert!(content.contains("Synced"));
    }

    #[test]
    fn badge_truncates_within_area() {
        let badge = Badge::new("Very long text here", BadgeVariant::Brand);
        let buf = render_badge(badge, 8);
        assert_eq!(buf.area.width, 8);
    }
}
