use macroquad::prelude::*;

use super::theme;

/// The message in a dark box, centred near the bottom of `area` (the board).
pub fn draw_toast(text: &str, area: Rect, font: &Font) {
    let size = measure_text(text, Some(font), theme::TOAST_FONT_SIZE, 1.0);
    let pad = theme::ABOUT_PADDING;
    let (w, h) = ((size.width + 2.0 * pad).min(area.w), size.height + 2.0 * pad);
    let x = area.center().x - w / 2.0;
    let y = area.bottom() - h - pad;

    draw_rectangle(x, y, w, h, theme::TOAST_BACKGROUND);
    let params = TextParams {
        font: Some(font),
        font_size: theme::TOAST_FONT_SIZE,
        color: theme::TOAST_TEXT,
        ..Default::default()
    };
    draw_text_ex(
        text,
        area.center().x - size.width / 2.0,
        y + pad + size.offset_y,
        params,
    );
}
