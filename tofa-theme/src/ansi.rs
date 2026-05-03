//! Raw ANSI escape helpers for CLI output. No external deps.

use crate::palette::{BRAND_RGB, DANGER_RGB, SUCCESS_RGB, TEXT_MUTED_RGB, WARNING_RGB};

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

pub fn fg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m")
}

pub fn brand() -> String {
    let (r, g, b) = BRAND_RGB;
    fg(r, g, b)
}
pub fn success() -> String {
    let (r, g, b) = SUCCESS_RGB;
    fg(r, g, b)
}
pub fn warning() -> String {
    let (r, g, b) = WARNING_RGB;
    fg(r, g, b)
}
pub fn danger() -> String {
    let (r, g, b) = DANGER_RGB;
    fg(r, g, b)
}
pub fn muted() -> String {
    let (r, g, b) = TEXT_MUTED_RGB;
    fg(r, g, b)
}
pub fn box_color() -> String {
    fg(117, 89, 184)
}

/// Color for a seconds-remaining value (same logic as palette::timer_color)
pub fn timer(seconds: u64) -> String {
    if seconds >= 10 {
        brand()
    } else if seconds >= 5 {
        warning()
    } else {
        danger()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fg_produces_correct_escape() {
        assert_eq!(fg(184, 158, 255), "\x1b[38;2;184;158;255m");
    }

    #[test]
    fn reset_is_correct() {
        assert_eq!(RESET, "\x1b[0m");
    }

    #[test]
    fn timer_above_10_is_brand() {
        assert_eq!(timer(15), brand());
    }

    #[test]
    fn timer_below_5_is_danger() {
        assert_eq!(timer(3), danger());
    }
}
