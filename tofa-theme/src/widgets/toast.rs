use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};
use crate::{palette, theme::ThemeMode};

#[derive(Clone, PartialEq, Eq)]
pub enum ToastVariant { Success, Error }

#[derive(Clone)]
pub struct Toast {
    pub message: String,
    pub variant: ToastVariant,
    pub theme:   ThemeMode,
}

impl Toast {
    pub fn success(msg: impl Into<String>) -> Self {
        Self { message: msg.into(), variant: ToastVariant::Success, theme: ThemeMode::Dark }
    }
    pub fn error(msg: impl Into<String>) -> Self {
        Self { message: msg.into(), variant: ToastVariant::Error, theme: ThemeMode::Dark }
    }
    pub fn theme(mut self, t: ThemeMode) -> Self { self.theme = t; self }
}

/// Bounded queue of active toasts. Push to add; the TUI tick should call `tick()`.
pub struct ToastQueue {
    inner: std::collections::VecDeque<(Toast, u8)>, // (toast, ticks_remaining)
}

impl ToastQueue {
    pub fn new() -> Self { Self { inner: std::collections::VecDeque::new() } }

    pub fn push(&mut self, toast: Toast) {
        if self.inner.len() >= 3 { self.inner.pop_front(); }
        self.inner.push_back((toast, 20)); // ~2s at ~10fps
    }

    /// Decrement counters and remove expired toasts. Call once per render tick.
    pub fn tick(&mut self) {
        self.inner.retain_mut(|(_, t)| { *t = t.saturating_sub(1); *t > 0 });
    }

    pub fn latest(&self) -> Option<&Toast> {
        self.inner.back().map(|(t, _)| t)
    }
}

impl Widget for Toast {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color = match self.variant {
            ToastVariant::Success => palette::SUCCESS,
            ToastVariant::Error   => palette::DANGER,
        };
        let text = format!(" {} ", self.message);
        let width = (text.len() as u16).min(area.width);
        let x = area.x + area.width.saturating_sub(width) / 2;
        let y = area.y + area.height.saturating_sub(3);
        let toast_area = Rect::new(x, y, width, 1);
        Clear.render(toast_area, buf);
        let line = Line::from(Span::styled(text, Style::default().fg(color)));
        Paragraph::new(line).render(toast_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_caps_at_3() {
        let mut q = ToastQueue::new();
        for i in 0..5 { q.push(Toast::success(format!("msg {i}"))); }
        assert!(q.inner.len() <= 3);
    }

    #[test]
    fn queue_tick_decrements() {
        let mut q = ToastQueue::new();
        q.push(Toast::success("hi"));
        let initial = q.inner.back().unwrap().1;
        q.tick();
        assert_eq!(q.inner.back().unwrap().1, initial - 1);
    }

    #[test]
    fn queue_tick_removes_expired() {
        let mut q = ToastQueue::new();
        q.inner.push_back((Toast::success("hi"), 1));
        q.tick();
        assert!(q.inner.is_empty());
    }
}
