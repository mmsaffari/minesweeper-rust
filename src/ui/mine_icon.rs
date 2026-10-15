use macroquad::prelude::*;

use super::theme;

/// A round black mine with four spikes and a white shine. Used on the board and in the
/// About dialog. `shine` scales the highlight: 1.0 is normal, more makes it glint.
pub fn draw_mine(center: Vec2, radius: f32, shine: f32) {
    let spike = radius * 1.5;
    let thickness = (radius * 0.33).max(2.0);
    let Vec2 { x, y } = center;
    draw_line(x - spike, y, x + spike, y, thickness, theme::MINE);
    draw_line(x, y - spike, x, y + spike, thickness, theme::MINE);
    // The diagonal spikes are a bit shorter, like on the original.
    let d = spike * 0.75;
    draw_line(x - d, y - d, x + d, y + d, thickness, theme::MINE);
    draw_line(x - d, y + d, x + d, y - d, thickness, theme::MINE);
    draw_circle(x, y, radius, theme::MINE);
    draw_circle(
        x - radius * 0.35,
        y - radius * 0.35,
        radius * 0.25 * shine,
        theme::MINE_SHINE,
    );
}
