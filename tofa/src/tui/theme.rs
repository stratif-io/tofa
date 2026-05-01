use ratatui::style::Color;

pub const BG:          Color = Color::Rgb(13,  17,  23);   // #0d1117
pub const SURFACE:     Color = Color::Rgb(10,  14,  20);   // #0a0e14
pub const BORDER:      Color = Color::Rgb(33,  38,  45);   // #21262d
pub const TEXT:        Color = Color::Rgb(230, 237, 243);  // #e6edf3
pub const DIM:         Color = Color::Rgb(110, 118, 129);  // #6e7681
pub const MUTED:       Color = Color::Rgb(48,  54,  61);   // #30363d
pub const SELECTED:    Color = Color::Rgb(48,  54,  61);   // #30363d - same as MUTED for highlight state
pub const ACCENT:      Color = Color::Rgb(88,  166, 255);  // #58a6ff
pub const CODE:        Color = Color::Rgb(121, 192, 255);  // #79c0ff
pub const WARNING:     Color = Color::Rgb(210, 153, 34);   // #d29922
pub const URGENT:      Color = Color::Rgb(248, 81,  73);   // #f85149
pub const BADGE_BG:    Color = Color::Rgb(31,  48,  88);   // #1f3058

/// Couleur du timer et du code selon les secondes restantes dans la fenêtre TOTP.
/// > 10s → CODE (bleu), 5–10s → WARNING (orange), < 5s → URGENT (rouge)
pub fn timer_color(seconds: u64) -> Color {
    match seconds {
        s if s > 10 => CODE,
        s if s >= 5 => WARNING,
        _           => URGENT,
    }
}
