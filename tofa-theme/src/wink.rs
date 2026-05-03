/// Large Sir Wink — Braille Unicode art, ~10×8 terminal chars.
/// Use on unlock/splash screens. Color with theme.brand().
pub const WINK_LARGE: &str = "\
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⠛⠛⣿⠻⠿⠿⣿⣿
⣿⣿⠃⣿⡟⣠⣤⠈⣿⣿
⣿⣿⠛⠛⣿⠛⠛⠛⣿⣿
⣿⡇⠀⣿⣿⡆⠀⠀⢸⣿
⣿⣿⣄⣀⣀⣀⣀⣄⣿⣿
⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛";

/// Small Sir Wink — block Unicode art, ~5×3 terminal chars.
/// Use inline in TUI header block title.
pub const WINK_SMALL: &str = "▄▄▄▄▄\n█▀▄▀█\n▀▀▀▀▀";

/// Width of WINK_LARGE in terminal columns (each Braille char = 1 column).
pub const WINK_LARGE_WIDTH: u16 = 10;
pub const WINK_LARGE_HEIGHT: u16 = 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wink_large_has_correct_line_count() {
        assert_eq!(WINK_LARGE.lines().count(), WINK_LARGE_HEIGHT as usize);
    }

    #[test]
    fn wink_large_lines_have_correct_width() {
        for line in WINK_LARGE.lines() {
            assert_eq!(line.chars().count(), WINK_LARGE_WIDTH as usize, "line: {line}");
        }
    }

    #[test]
    fn wink_small_has_three_lines() {
        assert_eq!(WINK_SMALL.lines().count(), 3);
    }
}
