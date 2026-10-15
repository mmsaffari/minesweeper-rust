//! The drawing every dialog shares: the dimmed window, the panel with its shadow and
//! title bar, push buttons, and text lined up in a row.

use macroquad::prelude::*;

use super::{bevel, theme};

/// Dims the window, then draws the raised panel and its title bar on top.
pub fn draw_dialog_frame(panel: Rect, title_bar: Rect, title: &str, font: &Font) {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), theme::DIALOG_SHADE);
    draw_rectangle(panel.x + 4.0, panel.y + 4.0, panel.w, panel.h, theme::DIALOG_SHADOW);
    bevel::draw_raised(panel, theme::BUTTON_BEVEL, theme::BACKGROUND);
    draw_title_bar(title_bar, title, font);
}

/// A navy-to-blue gradient, drawn as thin vertical strips (macroquad has no gradient fill).
fn draw_title_bar(bar: Rect, title: &str, font: &Font) {
    const STRIPS: usize = 32;
    let strip_w = bar.w / STRIPS as f32;
    for i in 0..STRIPS {
        let t = i as f32 / (STRIPS - 1) as f32;
        let color = mix(theme::TITLE_BAR_START, theme::TITLE_BAR_END, t);
        // One pixel wider than needed, so rounding never leaves a gap between strips.
        draw_rectangle(bar.x + i as f32 * strip_w, bar.y, strip_w + 1.0, bar.h, color);
    }
    let x = bar.x + theme::ABOUT_PADDING;
    draw_text_at(title, x, bar, theme::ABOUT_TITLE_FONT_SIZE, theme::TITLE_BAR_TEXT, font);
}

/// The way from colour `a` (t = 0) to colour `b` (t = 1).
fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

/// Raised normally; flat with a dark outline while held, and the label moves down a pixel,
/// so it feels like it's being pushed.
pub fn draw_button(rect: Rect, label: &str, pressed: bool, font: &Font) {
    let label_row = if pressed {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, theme::BACKGROUND);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, theme::BEVEL_DARK);
        Rect {
            y: rect.y + 1.0,
            ..rect
        }
    } else {
        bevel::draw_raised(rect, theme::BUTTON_BEVEL, theme::BACKGROUND);
        rect
    };
    draw_text_centered(label, label_row, 14, theme::MENU_TEXT, font);
}

/// Text centred in `row`, both ways.
pub fn draw_text_centered(text: &str, row: Rect, size: u16, color: Color, font: &Font) {
    let width = measure_text(text, Some(font), size, 1.0).width;
    draw_text_at(text, row.center().x - width / 2.0, row, size, color, font);
}

/// Text starting at `x`, vertically centred in `row`.
pub fn draw_text_at(text: &str, x: f32, row: Rect, size: u16, color: Color, font: &Font) {
    let dims = measure_text(text, Some(font), size, 1.0);
    let y = row.y + (row.h - dims.height) / 2.0 + dims.offset_y;
    let params = TextParams {
        font: Some(font),
        font_size: size,
        color,
        ..Default::default()
    };
    draw_text_ex(text, x, y, params);
}
