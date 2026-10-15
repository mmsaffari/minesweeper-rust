//! The classic 3D edges, used by the cells, the board frame, the counters and the face button.

use macroquad::prelude::*;

use super::theme;

/// A button that sticks out: light edges top/left, dark edges bottom/right, filled with `fill`.
pub fn draw_raised(rect: Rect, width: f32, fill: Color) {
    let Rect { x, y, w, h } = rect;
    draw_rectangle(x, y, w, h, fill);
    draw_rectangle(x, y, w, width, theme::BEVEL_LIGHT); // top
    draw_rectangle(x, y, width, h, theme::BEVEL_LIGHT); // left
    draw_rectangle(x, y + h - width, w, width, theme::BEVEL_DARK); // bottom
    draw_rectangle(x + w - width, y, width, h, theme::BEVEL_DARK); // right
}

/// A frame drawn just *outside* `rect` that makes it look set into the window:
/// dark edges top/left, light edges bottom/right (the opposite of raised).
pub fn draw_sunken_border(rect: Rect, width: f32) {
    let Rect { x, y, w, h } = Rect::new(
        rect.x - width,
        rect.y - width,
        rect.w + 2.0 * width,
        rect.h + 2.0 * width,
    );
    draw_rectangle(x, y, w, width, theme::BEVEL_DARK); // top
    draw_rectangle(x, y, width, h, theme::BEVEL_DARK); // left
    draw_rectangle(x, y + h - width, w, width, theme::BEVEL_LIGHT); // bottom
    draw_rectangle(x + w - width, y, width, h, theme::BEVEL_LIGHT); // right
}
