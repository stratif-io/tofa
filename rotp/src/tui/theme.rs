use ratatui::style::Color;

pub const BG: Color = Color::Rgb(13, 13, 13);
pub const GREEN: Color = Color::Rgb(0, 255, 65);
pub const DIM_GREEN: Color = Color::Rgb(0, 150, 38);
pub const DIM: Color = Color::Rgb(85, 85, 85);
pub const ORANGE: Color = Color::Rgb(255, 136, 0);
pub const RED: Color = Color::Rgb(255, 68, 68);
pub const WHITE: Color = Color::Rgb(220, 220, 220);

pub fn timer_color(seconds: u64) -> Color {
    match seconds {
        s if s > 20 => GREEN,
        s if s > 10 => ORANGE,
        _ => RED,
    }
}
