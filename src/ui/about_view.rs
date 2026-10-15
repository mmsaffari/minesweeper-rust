use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;

use super::{AboutLayout, bevel, mine_icon, theme};

/// Filled in by Cargo at compile time from `Cargo.toml`, so they can never be out of date.
const VERSION: &str = env!("CARGO_PKG_VERSION");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");

/// Draws the About dialog on top of everything, with the window dimmed behind it.
/// `time` (seconds since start, from `get_time()`) animates the mine's glint.
pub fn draw_about(layout: &AboutLayout, ok_pressed: bool, font: &Font, time: f64) {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), theme::DIALOG_SHADE);

    let panel = layout.panel;
    draw_rectangle(panel.x + 4.0, panel.y + 4.0, panel.w, panel.h, theme::DIALOG_SHADOW);
    bevel::draw_raised(panel, theme::BUTTON_BEVEL, theme::BACKGROUND);

    draw_title_bar(layout.title_bar, font);
    draw_glinting_mine(layout.icon, time);
    draw_lines(layout.text, font);
    draw_ok_button(layout.ok_button, ok_pressed, font);
}

/// A navy-to-blue gradient, drawn as thin vertical strips (macroquad has no gradient fill).
fn draw_title_bar(bar: Rect, font: &Font) {
    const STRIPS: usize = 32;
    let strip_w = bar.w / STRIPS as f32;
    for i in 0..STRIPS {
        let t = i as f32 / (STRIPS - 1) as f32;
        let color = mix(theme::TITLE_BAR_START, theme::TITLE_BAR_END, t);
        // One pixel wider than needed, so rounding never leaves a gap between strips.
        draw_rectangle(bar.x + i as f32 * strip_w, bar.y, strip_w + 1.0, bar.h, color);
    }
    let x = bar.x + theme::ABOUT_PADDING;
    draw_line_of_text(
        "About Minesweeper",
        x,
        bar,
        theme::ABOUT_TITLE_FONT_SIZE,
        theme::TITLE_BAR_TEXT,
        font,
    );
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

/// The shiny part: the mine's highlight swells and shrinks, and a little star next to it
/// turns slowly and brightens with it.
fn draw_glinting_mine(icon: Rect, time: f64) {
    let pulse = (time * 3.0).sin() as f32; // swings between -1 and 1, about twice a second
    let center = icon.center();
    let radius = icon.h * 0.3;
    mine_icon::draw_mine(center, radius, 1.15 + 0.35 * pulse);

    let at = center + vec2(radius * 1.1, -radius * 1.1);
    let size = radius * (0.45 + 0.15 * pulse);
    let angle = (time * 1.5) as f32;
    let sparkle = Color {
        a: 0.55 + 0.45 * pulse.max(0.0),
        ..theme::SPARKLE
    };
    for arm in 0..2 {
        let a = angle + arm as f32 * FRAC_PI_2; // two lines at right angles make four points
        let d = vec2(a.cos(), a.sin()) * size;
        draw_line(at.x - d.x, at.y - d.y, at.x + d.x, at.y + d.y, 2.0, sparkle);
    }
}

fn draw_lines(area: Rect, font: &Font) {
    let lines = [
        ("Minesweeper".to_owned(), 22, theme::MENU_TEXT),
        (format!("Version {VERSION}"), 14, theme::MENU_DIM_TEXT),
        ("The classic, rebuilt in Rust".to_owned(), 12, theme::MENU_TEXT),
        ("© 2026 Mahdi Saffari".to_owned(), 12, theme::MENU_TEXT),
        (format!("License: {LICENSE}"), 12, theme::MENU_DIM_TEXT),
        ("macroquad, Roboto, DSEG7".to_owned(), 12, theme::MENU_DIM_TEXT),
    ];
    let row_h = area.h / lines.len() as f32;
    for (i, (text, size, color)) in lines.iter().enumerate() {
        let row = Rect::new(area.x, area.y + i as f32 * row_h, area.w, row_h);
        draw_centered(text, row, *size, *color, font);
    }
}

/// Raised normally; flat with a dark outline while held, and the label moves down a pixel,
/// so it feels like it's being pushed.
fn draw_ok_button(rect: Rect, pressed: bool, font: &Font) {
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
    draw_centered("OK", label_row, 14, theme::MENU_TEXT, font);
}

fn draw_centered(text: &str, row: Rect, size: u16, color: Color, font: &Font) {
    let width = measure_text(text, Some(font), size, 1.0).width;
    draw_line_of_text(text, row.center().x - width / 2.0, row, size, color, font);
}

/// Text starting at `x`, vertically centred in `row`.
fn draw_line_of_text(text: &str, x: f32, row: Rect, size: u16, color: Color, font: &Font) {
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
