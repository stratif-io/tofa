use crate::theme::theme::ThemeMode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Gauge, Paragraph, Widget},
};

pub struct OtpDisplay {
    pub issuer: String,
    pub account: String,
    pub code: String, // formatted, e.g. "847 392"
    pub seconds: u64, // seconds remaining in TOTP window
    pub period: u64,  // TOTP period (usually 30)
    pub focused: bool,
    pub theme: ThemeMode,
}

impl OtpDisplay {
    pub fn new(
        issuer: impl Into<String>,
        account: impl Into<String>,
        code: impl Into<String>,
        seconds: u64,
        period: u64,
    ) -> Self {
        Self {
            issuer: issuer.into(),
            account: account.into(),
            code: code.into(),
            seconds,
            period,
            focused: false,
            theme: ThemeMode::Dark,
        }
    }
    pub fn focused(mut self, f: bool) -> Self {
        self.focused = f;
        self
    }
    pub fn theme(mut self, t: ThemeMode) -> Self {
        self.theme = t;
        self
    }
}

impl Widget for OtpDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let timer_color = self.theme.timer_color(self.seconds);
        let brand = self.theme.brand();
        let muted = self.theme.text_muted();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // issuer · account
                Constraint::Length(1), // code
                Constraint::Length(1), // Xs remaining
                Constraint::Length(1), // progress bar
            ])
            .split(area);

        // Issuer · account
        let header = format!("{} · {}", self.issuer, self.account);
        Paragraph::new(Line::from(Span::styled(header, Style::default().fg(muted))))
            .render(chunks[0], buf);

        // OTP code
        Paragraph::new(Line::from(Span::styled(
            self.code.clone(),
            Style::default()
                .fg(if self.focused { brand } else { timer_color })
                .add_modifier(Modifier::BOLD),
        )))
        .render(chunks[1], buf);

        // "Xs remaining" right-aligned
        let label = format!("{}s remaining", self.seconds);
        let label_x = chunks[2].x + chunks[2].width.saturating_sub(label.len() as u16);
        let label_area = Rect::new(label_x, chunks[2].y, label.len() as u16, 1);
        Paragraph::new(Line::from(Span::styled(
            label,
            Style::default().fg(timer_color),
        )))
        .render(label_area, buf);

        // Progress bar
        let ratio = (self.seconds as f64 / self.period.max(1) as f64).clamp(0.0, 1.0);
        Gauge::default()
            .ratio(ratio)
            .gauge_style(Style::default().fg(timer_color).bg(self.theme.border()))
            .render(chunks[3], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    fn render_otp(seconds: u64) -> String {
        let backend = TestBackend::new(40, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let w = OtpDisplay::new("GitHub", "user@ex.com", "847 392", seconds, 30);
                f.render_widget(w, f.area());
            })
            .unwrap();
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|c| c.symbol().to_string())
            .collect()
    }

    #[test]
    fn renders_issuer_and_code() {
        let content = render_otp(20);
        assert!(content.contains("GitHub"));
        assert!(content.contains("847 392"));
    }

    #[test]
    fn renders_seconds_label() {
        let content = render_otp(19);
        assert!(content.contains("19s remaining"));
    }
}
