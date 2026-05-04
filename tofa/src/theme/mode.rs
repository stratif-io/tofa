use crate::theme::palette;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    /// Detect terminal theme from env vars, falling back to Dark.
    /// Priority: $TOFA_THEME → $TERM_BACKGROUND → $COLORSCHEME → Dark
    pub fn detect() -> Self {
        for var in &["TOFA_THEME", "TERM_BACKGROUND", "COLORSCHEME"] {
            if let Ok(val) = std::env::var(var) {
                let val = val.to_lowercase();
                if val.contains("light") {
                    return Self::Light;
                }
                if val.contains("dark") {
                    return Self::Dark;
                }
            }
        }
        Self::Dark
    }

    pub fn brand(&self) -> Color {
        match self {
            Self::Dark => palette::BRAND,
            Self::Light => palette::BRAND_LIGHT,
        }
    }
    pub fn bg(&self) -> Color {
        match self {
            Self::Dark => palette::BG,
            Self::Light => palette::BG_LIGHT,
        }
    }
    pub fn surface(&self) -> Color {
        match self {
            Self::Dark => palette::SURFACE,
            Self::Light => palette::SURFACE_LIGHT,
        }
    }
    pub fn border(&self) -> Color {
        match self {
            Self::Dark => palette::BORDER,
            Self::Light => palette::BORDER_LIGHT,
        }
    }
    pub fn text(&self) -> Color {
        match self {
            Self::Dark => palette::TEXT,
            Self::Light => palette::TEXT_LIGHT,
        }
    }
    pub fn text_muted(&self) -> Color {
        match self {
            Self::Dark => palette::TEXT_MUTED,
            Self::Light => palette::TEXT_MUTED_LIGHT,
        }
    }
    pub fn timer_color(&self, seconds: u64) -> Color {
        match self {
            Self::Dark => palette::timer_color(seconds),
            Self::Light => palette::timer_color_light(seconds),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_is_default_when_no_env() {
        std::env::remove_var("TOFA_THEME");
        std::env::remove_var("TERM_BACKGROUND");
        std::env::remove_var("COLORSCHEME");
        assert_eq!(ThemeMode::detect(), ThemeMode::Dark);
    }

    #[test]
    fn detects_light_from_tofa_theme() {
        std::env::set_var("TOFA_THEME", "light");
        let mode = ThemeMode::detect();
        std::env::remove_var("TOFA_THEME");
        assert_eq!(mode, ThemeMode::Light);
    }

    #[test]
    fn detects_dark_from_term_background() {
        std::env::remove_var("TOFA_THEME");
        std::env::set_var("TERM_BACKGROUND", "dark");
        let mode = ThemeMode::detect();
        std::env::remove_var("TERM_BACKGROUND");
        assert_eq!(mode, ThemeMode::Dark);
    }

    #[test]
    fn brand_differs_by_mode() {
        assert_ne!(ThemeMode::Dark.brand(), ThemeMode::Light.brand());
    }
}
