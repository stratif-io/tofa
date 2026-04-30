use ratatui::style::Color;

pub const BG:       Color = Color::Rgb(17,  17,  17);   // #111111
pub const SURFACE:  Color = Color::Rgb(28,  28,  28);   // #1C1C1C
pub const SELECTED: Color = Color::Rgb(30,  26,  46);   // #1E1A2E
pub const TEXT:     Color = Color::Rgb(224, 224, 224);  // #E0E0E0
pub const DIM:      Color = Color::Rgb(136, 136, 136);  // #888888
pub const MUTED:    Color = Color::Rgb(80,  80,  80);   // #505050
pub const ACCENT:   Color = Color::Rgb(167, 139, 250);  // #A78BFA
pub const WARNING:  Color = Color::Rgb(251, 146, 60);   // #FB923C
pub const URGENT:   Color = Color::Rgb(248, 113, 113);  // #F87171
pub const BORDER:   Color = Color::Rgb(42,  42,  42);   // #2A2A2A

/// Returns the timer colour based on seconds remaining in the 30s TOTP window.
/// > 10s → ACCENT, 5–10s → WARNING, < 5s → URGENT
pub fn timer_color(seconds: u64) -> Color {
    match seconds {
        s if s > 10 => ACCENT,
        s if s >= 5 => WARNING,
        _           => URGENT,
    }
}
