//! The classic Windows Minesweeper look: every size and colour in one place.

use macroquad::color::Color;

/// The original was 16 px, which is tiny on today's screens.
pub const CELL_SIZE: f32 = 24.0;
/// Width of the light/dark edge that makes a hidden cell look raised.
pub const BEVEL: f32 = 3.0;
/// Width of the line between opened cells.
pub const GRID_LINE: f32 = 1.0;
/// For numbers, flags and question marks. `measure_text` wants a `u16`.
pub const FONT_SIZE: u16 = 22;

pub const BACKGROUND: Color = Color::from_rgba(0xC0, 0xC0, 0xC0, 0xFF);
pub const BEVEL_LIGHT: Color = Color::from_rgba(0xFF, 0xFF, 0xFF, 0xFF);
pub const BEVEL_DARK: Color = Color::from_rgba(0x80, 0x80, 0x80, 0xFF);
pub const GRID: Color = BEVEL_DARK;

pub const FLAG: Color = Color::from_rgba(0xFF, 0x00, 0x00, 0xFF);
pub const QUESTION_MARK: Color = Color::from_rgba(0x00, 0x00, 0x00, 0xFF);
pub const MINE: Color = Color::from_rgba(0x00, 0x00, 0x00, 0xFF);
pub const MINE_SHINE: Color = Color::from_rgba(0xFF, 0xFF, 0xFF, 0xFF);
/// Background of the mine that was clicked, and the cross over a wrong flag.
pub const EXPLODED: Color = Color::from_rgba(0xFF, 0x00, 0x00, 0xFF);

/// The classic colour for each number. 0 is never drawn.
pub fn number_color(adjacent_mines: u8) -> Color {
    let (r, g, b) = match adjacent_mines {
        1 => (0x00, 0x00, 0xFF), // blue
        2 => (0x00, 0x80, 0x00), // green
        3 => (0xFF, 0x00, 0x00), // red
        4 => (0x00, 0x00, 0x80), // navy
        5 => (0x80, 0x00, 0x00), // maroon
        6 => (0x00, 0x80, 0x80), // teal
        7 => (0x00, 0x00, 0x00), // black
        _ => (0x80, 0x80, 0x80), // 8: grey
    };
    Color::from_rgba(r, g, b, 0xFF)
}
