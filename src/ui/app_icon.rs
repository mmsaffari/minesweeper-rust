//! The game's icon, a mine on a raised grey cell, drawn pixel by pixel.
//!
//! Used twice: as the window icon (`main.rs`), and through `build.rs` as the `.exe` icon
//! on Windows. `build.rs` includes this file directly (it can't use the crate), so this
//! file may only use `std`, never `crate::` or `super::`.
//!
//! `assets/icon.png` (shown in the README) is a 128 px export of this drawing. It's the one
//! copy that isn't generated automatically: if the drawing changes, export it again.

const LIGHT: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
const DARK: [u8; 4] = [0x80, 0x80, 0x80, 0xFF];
const FACE: [u8; 4] = [0xC0, 0xC0, 0xC0, 0xFF];
const MINE: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
const SHINE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

/// Each pixel is sampled 4 × 4 times, so edges come out smooth (anti-aliased) even at 16 px.
const SAMPLES: u32 = 4;

/// RGBA pixels, row by row from the top, for a `size` × `size` icon.
pub fn app_icon_rgba(size: u32) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((size * size * 4) as usize);
    for y in 0..size {
        for x in 0..size {
            pixels.extend_from_slice(&pixel(x, y, size));
        }
    }
    pixels
}

fn pixel(x: u32, y: u32, size: u32) -> [u8; 4] {
    // How much of this pixel the mine and its shine cover, from 0.0 to 1.0.
    let (mut mine, mut shine) = (0, 0);
    for sy in 0..SAMPLES {
        for sx in 0..SAMPLES {
            // The sample's position in the icon, from 0.0 to 1.0 on each axis.
            let u = (x as f32 + (sx as f32 + 0.5) / SAMPLES as f32) / size as f32;
            let v = (y as f32 + (sy as f32 + 0.5) / SAMPLES as f32) / size as f32;
            if in_shine(u, v) {
                shine += 1;
            } else if in_mine(u, v) {
                mine += 1;
            }
        }
    }
    let total = (SAMPLES * SAMPLES) as f32;
    let color = mix(cell(x, y, size), MINE, mine as f32 / total);
    mix(color, SHINE, shine as f32 / total)
}

/// The raised cell behind the mine: light edges top/left, dark edges bottom/right.
fn cell(x: u32, y: u32, size: u32) -> [u8; 4] {
    let bevel = (size / 16).max(1);
    let top_left = x < bevel || y < bevel;
    let bottom_right = x >= size - bevel || y >= size - bevel;
    match (top_left, bottom_right) {
        // The top-right and bottom-left corners belong to both edges: split them diagonally.
        (true, true) => {
            if x + y < size - 1 {
                LIGHT
            } else {
                DARK
            }
        }
        (true, false) => LIGHT,
        (false, true) => DARK,
        (false, false) => FACE,
    }
}

/// The mine's body and its 8 spikes (the diagonal ones a bit shorter), in 0..1 coordinates.
fn in_mine(u: f32, v: f32) -> bool {
    let (c, body, spike, diagonal, half_thickness) = (0.5, 0.22, 0.36, 0.27, 0.045);
    let near = |(x1, y1): (f32, f32), (x2, y2): (f32, f32)| distance_to_segment(u, v, x1, y1, x2, y2) <= half_thickness;
    distance(u, v, c, c) <= body
        || near((c - spike, c), (c + spike, c))
        || near((c, c - spike), (c, c + spike))
        || near((c - diagonal, c - diagonal), (c + diagonal, c + diagonal))
        || near((c - diagonal, c + diagonal), (c + diagonal, c - diagonal))
}

/// The white highlight on the upper left of the mine.
fn in_shine(u: f32, v: f32) -> bool {
    distance(u, v, 0.42, 0.42) <= 0.06
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// The shortest distance from point (px, py) to the line segment from (x1, y1) to (x2, y2).
fn distance_to_segment(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let (dx, dy) = (x2 - x1, y2 - y1);
    // How far along the segment the closest point is: 0 = at the start, 1 = at the end.
    let t = (((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    distance(px, py, x1 + t * dx, y1 + t * dy)
}

/// `a` with `amount` (0.0 to 1.0) of `b` mixed in.
fn mix(a: [u8; 4], b: [u8; 4], amount: f32) -> [u8; 4] {
    let channel = |i: usize| (f32::from(a[i]) * (1.0 - amount) + f32::from(b[i]) * amount).round() as u8;
    [channel(0), channel(1), channel(2), channel(3)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pixel at (x, y) of a `size` icon.
    fn at(size: u32, x: u32, y: u32) -> [u8; 4] {
        let pixels = app_icon_rgba(size);
        let i = ((y * size + x) * 4) as usize;
        [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
    }

    fn brightness(pixel: [u8; 4]) -> u32 {
        pixel[..3].iter().map(|&c| u32::from(c)).sum()
    }

    #[test]
    fn has_four_bytes_per_pixel_for_every_size_windows_asks_for() {
        for size in [16, 24, 32, 48, 64, 128] {
            assert_eq!(app_icon_rgba(size).len(), (size * size * 4) as usize, "{size} px");
        }
    }

    #[test]
    fn is_fully_opaque() {
        assert!(app_icon_rgba(32).chunks(4).all(|pixel| pixel[3] == 0xFF));
    }

    #[test]
    fn the_centre_is_the_black_mine() {
        for size in [16, 32, 64] {
            assert_eq!(at(size, size / 2, size / 2), MINE, "{size} px");
        }
    }

    #[test]
    fn the_cell_is_raised_light_top_left_dark_bottom_right() {
        assert_eq!(at(32, 0, 0), LIGHT);
        assert_eq!(at(32, 31, 31), DARK);
        assert_eq!(at(32, 4, 4), FACE, "the cell's face, outside the mine");
    }

    #[test]
    fn the_shine_is_brighter_than_the_mine_around_it() {
        let shine = at(64, 27, 27); // 0.42 of 64
        let body = at(64, 40, 40);
        assert!(brightness(shine) > brightness(body) + 300, "{shine:?} vs {body:?}");
    }
}
