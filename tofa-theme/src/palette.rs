use ratatui::style::Color;

// Raw RGB tuples — use these for crossterm in CLI
pub const BRAND_RGB: (u8, u8, u8) = (184, 158, 255);
pub const SUCCESS_RGB: (u8, u8, u8) = (74, 222, 128);
pub const WARNING_RGB: (u8, u8, u8) = (251, 191, 36);
pub const DANGER_RGB: (u8, u8, u8) = (248, 113, 113);
pub const TEXT_RGB: (u8, u8, u8) = (232, 230, 240);
pub const TEXT_MUTED_RGB: (u8, u8, u8) = (122, 118, 144);
pub const BG_RGB: (u8, u8, u8) = (10, 10, 18);
pub const SURFACE_RGB: (u8, u8, u8) = (20, 19, 31);
pub const BORDER_RGB: (u8, u8, u8) = (42, 42, 58);

// ratatui Color constants — dark mode defaults
pub const BRAND: Color = Color::Rgb(184, 158, 255);
pub const SUCCESS: Color = Color::Rgb(74, 222, 128);
pub const WARNING: Color = Color::Rgb(251, 191, 36);
pub const DANGER: Color = Color::Rgb(248, 113, 113);
pub const TEXT: Color = Color::Rgb(232, 230, 240);
pub const TEXT_MUTED: Color = Color::Rgb(122, 118, 144);
pub const BG: Color = Color::Rgb(10, 10, 18);
pub const SURFACE: Color = Color::Rgb(20, 19, 31);
pub const BORDER: Color = Color::Rgb(42, 42, 58);

// Light mode overrides
pub const BRAND_LIGHT: Color = Color::Rgb(117, 89, 184);
pub const TEXT_LIGHT: Color = Color::Rgb(10, 10, 18);
pub const TEXT_MUTED_LIGHT: Color = Color::Rgb(86, 83, 107);
pub const BG_LIGHT: Color = Color::Rgb(244, 243, 248);
pub const SURFACE_LIGHT: Color = Color::Rgb(255, 255, 255);
pub const BORDER_LIGHT: Color = Color::Rgb(216, 212, 224);

/// Progress bar color based on TOTP seconds remaining.
/// >= 10s → BRAND, < 10s → WARNING, < 5s → DANGER
pub fn timer_color(seconds: u64) -> Color {
    match seconds {
        s if s >= 10 => BRAND,
        s if s >= 5 => WARNING,
        _ => DANGER,
    }
}

/// Same logic for light mode
pub fn timer_color_light(seconds: u64) -> Color {
    match seconds {
        s if s >= 10 => BRAND_LIGHT,
        s if s >= 5 => WARNING,
        _ => DANGER,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_color_above_10s() {
        assert_eq!(timer_color(11), BRAND);
        assert_eq!(timer_color(10), BRAND);
    }

    #[test]
    fn timer_color_warning_band() {
        assert_eq!(timer_color(9), WARNING);
        assert_eq!(timer_color(5), WARNING);
    }

    #[test]
    fn timer_color_danger_band() {
        assert_eq!(timer_color(4), DANGER);
        assert_eq!(timer_color(0), DANGER);
    }
}
