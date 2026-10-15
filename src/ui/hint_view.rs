use macroquad::prelude::*;

use super::theme;

/// A pulsing outline around the hinted cell: green for safe, red for a mine.
/// `time` (from `get_time()`) makes it pulse, so it catches the eye on a big board.
pub fn draw_hint(cell: Rect, safe: bool, time: f64) {
    let base = if safe { theme::HINT_SAFE } else { theme::HINT_MINE };
    let pulse = 0.5 + 0.5 * (time * 5.0).sin() as f32; // between 0 and 1
    let color = Color {
        a: 0.55 + 0.45 * pulse,
        ..base
    };
    let t = theme::HINT_THICKNESS;
    draw_rectangle_lines(cell.x, cell.y, cell.w, cell.h, t, color);
}
