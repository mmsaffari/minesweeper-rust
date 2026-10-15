use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;

use super::dialog_view::{draw_button, draw_dialog_frame, draw_text_centered};
use super::{AboutLayout, mine_icon, theme};

/// Filled in by Cargo at compile time from `Cargo.toml`, so they can never be out of date.
const VERSION: &str = env!("CARGO_PKG_VERSION");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");

/// Draws the About dialog on top of everything, with the window dimmed behind it.
/// `time` (seconds since start, from `get_time()`) animates the mine's glint.
pub fn draw_about(layout: &AboutLayout, ok_pressed: bool, font: &Font, time: f64) {
    draw_dialog_frame(layout.panel, layout.title_bar, "About Minesweeper", font);
    draw_glinting_mine(layout.icon, time);
    draw_lines(layout.text, font);
    draw_button(layout.ok_button, "OK", ok_pressed, font);
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
        draw_text_centered(text, row, *size, *color, font);
    }
}
