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
    use std::sync::Mutex;

    // Tests below mutate process-global env vars. cargo runs tests in
    // parallel by default, so without a shared lock one test's set_var
    // leaks into another's detect() and the assertion races.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_clean_env<F: FnOnce()>(f: F) {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
        for var in ["TOFA_THEME", "TERM_BACKGROUND", "COLORSCHEME"] {
            std::env::remove_var(var);
        }
        f();
        for var in ["TOFA_THEME", "TERM_BACKGROUND", "COLORSCHEME"] {
            std::env::remove_var(var);
        }
    }

    #[test]
    fn dark_is_default_when_no_env() {
        with_clean_env(|| {
            assert_eq!(ThemeMode::detect(), ThemeMode::Dark);
        });
    }

    #[test]
    fn detects_light_from_tofa_theme() {
        with_clean_env(|| {
            std::env::set_var("TOFA_THEME", "light");
            assert_eq!(ThemeMode::detect(), ThemeMode::Light);
        });
    }

    #[test]
    fn detects_dark_from_term_background() {
        with_clean_env(|| {
            std::env::set_var("TERM_BACKGROUND", "dark");
            assert_eq!(ThemeMode::detect(), ThemeMode::Dark);
        });
    }

    #[test]
    fn brand_differs_by_mode() {
        assert_ne!(ThemeMode::Dark.brand(), ThemeMode::Light.brand());
    }
}
