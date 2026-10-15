use std::time::Instant;

use macroquad::prelude::*;

use super::{Face, HeaderLayout, bevel, led_digits, theme};
use crate::logic::Game;

/// Draws the mine counter, the face button and the timer. Only reads the game.
pub fn draw_header(game: &Game, header: &HeaderLayout, face: Face, led_font: &Font) {
    draw_led(header.counter, &led_digits(game.mines_left()), led_font);
    draw_face(header.face, face);

    // `elapsed_seconds` is at most 999, so this always fits; `unwrap_or` just avoids a panic path.
    let seconds = isize::try_from(game.elapsed_seconds(Instant::now())).unwrap_or(isize::MAX);
    draw_led(header.timer, &led_digits(seconds), led_font);
}

/// A red 7-segment display: the unlit segments ("888") faintly first, then the lit digits on top,
/// so it looks like a real LED panel rather than text on black.
fn draw_led(rect: Rect, digits: &str, font: &Font) {
    bevel::draw_sunken_border(rect, theme::LED_BEVEL);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, theme::LED_BACKGROUND);

    // DSEG7 gives every digit (and the minus) the same width, so "888" measures any 3 characters.
    let size = measure_text("888", Some(font), theme::LED_FONT_SIZE, 1.0);
    let x = rect.x + (rect.w - size.width) / 2.0;
    let y = rect.y + (rect.h - size.height) / 2.0 + size.offset_y;
    for (text, color) in [("888", theme::LED_OFF), (digits, theme::LED_ON)] {
        let params = TextParams {
            font: Some(font),
            font_size: theme::LED_FONT_SIZE,
            color,
            ..Default::default()
        };
        draw_text_ex(text, x, y, params);
    }
}

/// The new-game button: a raised square with a yellow smiley whose expression depends on `face`.
fn draw_face(rect: Rect, face: Face) {
    bevel::draw_raised(rect, theme::BUTTON_BEVEL, theme::BACKGROUND);

    let c = rect.center();
    let r = rect.w * 0.32;
    draw_circle(c.x, c.y, r, theme::FACE_YELLOW);
    draw_circle_lines(c.x, c.y, r, 1.5, theme::FACE_INK);

    let eye_y = c.y - r * 0.25;
    let eyes = [vec2(c.x - r * 0.38, eye_y), vec2(c.x + r * 0.38, eye_y)];
    let smile = || draw_arc(c, r * 0.55, 30.0, 150.0);

    match face {
        Face::Smile => {
            draw_eye_dots(eyes, r);
            smile();
        }
        Face::Surprised => {
            draw_eye_dots(eyes, r);
            draw_circle_lines(c.x, c.y + r * 0.45, r * 0.2, 1.5, theme::FACE_INK); // an "o"
        }
        Face::Cool => {
            draw_sunglasses(eyes, c, r);
            smile();
        }
        Face::Dead => {
            for eye in eyes {
                draw_x(eye, r * 0.15);
            }
            draw_arc(c + vec2(0.0, r * 0.85), r * 0.45, 210.0, 330.0); // a frown: the top of a circle
        }
    }
}

fn draw_eye_dots(eyes: [Vec2; 2], r: f32) {
    for eye in eyes {
        draw_circle(eye.x, eye.y, r * 0.11, theme::FACE_INK);
    }
}

fn draw_sunglasses(eyes: [Vec2; 2], c: Vec2, r: f32) {
    let (w, h) = (r * 0.5, r * 0.32);
    for eye in eyes {
        draw_rectangle(eye.x - w / 2.0, eye.y - h / 2.0, w, h, theme::FACE_INK);
    }
    draw_line(eyes[0].x, eyes[0].y, eyes[1].x, eyes[1].y, 1.5, theme::FACE_INK); // bridge
    draw_line(
        c.x - r * 0.95,
        c.y - r * 0.4,
        eyes[0].x,
        eyes[0].y,
        1.5,
        theme::FACE_INK,
    ); // left arm
    draw_line(
        c.x + r * 0.95,
        c.y - r * 0.4,
        eyes[1].x,
        eyes[1].y,
        1.5,
        theme::FACE_INK,
    ); // right arm
}

fn draw_x(center: Vec2, half: f32) {
    let Vec2 { x, y } = center;
    draw_line(x - half, y - half, x + half, y + half, 1.5, theme::FACE_INK);
    draw_line(x - half, y + half, x + half, y - half, 1.5, theme::FACE_INK);
}

/// Part of a circle, as short straight lines. Angles in degrees, 0 = right, 90 = down
/// (screen y grows downwards), so 30..150 is the bottom of the circle: a smile.
fn draw_arc(center: Vec2, radius: f32, from_deg: f32, to_deg: f32) {
    const STEPS: usize = 8;
    let point = |i: usize| {
        let degrees = from_deg + (to_deg - from_deg) * i as f32 / STEPS as f32;
        let angle = degrees.to_radians();
        center + vec2(angle.cos(), angle.sin()) * radius
    };
    for i in 0..STEPS {
        let (a, b) = (point(i), point(i + 1));
        draw_line(a.x, a.y, b.x, b.y, 1.5, theme::FACE_INK);
    }
}
