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

pub const SIDE_MARGIN: f32 = 12.0;
/// The menu bar along the top of the window.
pub const MENU_BAR_HEIGHT: f32 = 22.0;
/// Menu bar + the header strip (counter, face, timer) above the board.
pub const TOP_MARGIN: f32 = MENU_BAR_HEIGHT + 60.0;
pub const BOTTOM_MARGIN: f32 = 12.0;

/// The strip above the board: counter, face button, timer.
pub const HEADER_HEIGHT: f32 = 36.0;
/// Width of the mine counter and the timer (3 digits each).
pub const LED_WIDTH: f32 = 64.0;
pub const LED_FONT_SIZE: u16 = 24;
pub const LED_BEVEL: f32 = 2.0;
pub const BUTTON_BEVEL: f32 = 2.0;

pub const MENU_FONT_SIZE: u16 = 15;
pub const MENU_TITLE_WIDTH: f32 = 54.0;
pub const MENU_WIDTH: f32 = 220.0;
pub const MENU_ITEM_HEIGHT: f32 = 24.0;
/// Space on the left and right inside the menu.
pub const MENU_PADDING: f32 = 8.0;
/// Room on the left of each item for the check mark.
pub const MENU_CHECK_WIDTH: f32 = 26.0;
/// Room on the right of each item for its shortcut key.
pub const MENU_SHORTCUT_WIDTH: f32 = 24.0;

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

pub const LED_BACKGROUND: Color = Color::from_rgba(0x00, 0x00, 0x00, 0xFF);
pub const LED_ON: Color = Color::from_rgba(0xFF, 0x00, 0x00, 0xFF);
/// The unlit segments, faintly visible like on a real LED display.
pub const LED_OFF: Color = Color::from_rgba(0x40, 0x00, 0x00, 0xFF);
pub const FACE_YELLOW: Color = Color::from_rgba(0xFF, 0xFF, 0x00, 0xFF);
pub const FACE_INK: Color = Color::from_rgba(0x00, 0x00, 0x00, 0xFF);
pub const MENU_TEXT: Color = Color::from_rgba(0x00, 0x00, 0x00, 0xFF);
/// The classic Windows selection colour: navy, with white text.
pub const MENU_HIGHLIGHT: Color = Color::from_rgba(0x00, 0x00, 0x80, 0xFF);
pub const MENU_HIGHLIGHT_TEXT: Color = Color::from_rgba(0xFF, 0xFF, 0xFF, 0xFF);
/// The best times in the menu: quieter than the labels.
pub const MENU_DIM_TEXT: Color = Color::from_rgba(0x60, 0x60, 0x60, 0xFF);

// --- About dialog ---

pub const ABOUT_MAX_WIDTH: f32 = 300.0;
pub const ABOUT_HEIGHT: f32 = 260.0;
pub const ABOUT_PADDING: f32 = 10.0;
pub const ABOUT_TITLE_HEIGHT: f32 = 22.0;
pub const ABOUT_TITLE_FONT_SIZE: u16 = 14;
pub const ABOUT_ICON_HEIGHT: f32 = 50.0;
pub const ABOUT_BUTTON_WIDTH: f32 = 72.0;
pub const ABOUT_BUTTON_HEIGHT: f32 = 26.0;
/// Laid over the whole window while a dialog is open, so the dialog stands out.
pub const DIALOG_SHADE: Color = Color::new(0.0, 0.0, 0.0, 0.35);
pub const DIALOG_SHADOW: Color = Color::new(0.0, 0.0, 0.0, 0.4);
/// The Windows 98 title bar: a gradient from navy to light blue, with white text.
pub const TITLE_BAR_START: Color = Color::from_rgba(0x00, 0x00, 0x80, 0xFF);
pub const TITLE_BAR_END: Color = Color::from_rgba(0x10, 0x84, 0xD0, 0xFF);
pub const TITLE_BAR_TEXT: Color = Color::from_rgba(0xFF, 0xFF, 0xFF, 0xFF);
/// The little star that glints next to the mine in the About dialog.
pub const SPARKLE: Color = Color::from_rgba(0xFF, 0xE0, 0x40, 0xFF);
// --- Hint ---

pub const HINT_SAFE: Color = Color::from_rgba(0x00, 0xB0, 0x00, 0xFF);
pub const HINT_MINE: Color = Color::from_rgba(0xE0, 0x00, 0x00, 0xFF);
pub const HINT_THICKNESS: f32 = 3.0;

// --- Toasts (short messages over the board) ---

pub const TOAST_FONT_SIZE: u16 = 14;
pub const TOAST_BACKGROUND: Color = Color::new(0.0, 0.0, 0.0, 0.75);
pub const TOAST_TEXT: Color = Color::from_rgba(0xFF, 0xFF, 0xFF, 0xFF);

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
