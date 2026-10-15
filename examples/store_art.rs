//! Draws the images for the Store listing page (uploaded by hand in Partner Center, not part
//! of the package): the 9:16 poster art, the 1:1 box art, the 16:9 hero art and the app
//! tile icons.
//!
//! ```text
//! cargo run --example store_art -- <output folder>
//! ```
//!
//! Everything is laid out in "design units" for the biggest size, then multiplied by
//! `scale` for the smaller one, so both sizes are the same picture.

use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

use fontdue::{Font, FontSettings};

#[path = "../src/ui/app_icon.rs"]
mod app_icon;

type Rgb = [u8; 3];

// The game's colours (src/ui/theme.rs).
const FACE: Rgb = [0xC0, 0xC0, 0xC0];
const LIGHT: Rgb = [0xFF, 0xFF, 0xFF];
const DARK: Rgb = [0x80, 0x80, 0x80];
const BLUE: Rgb = [0x00, 0x00, 0xFF];
const GREEN: Rgb = [0x00, 0x80, 0x00];
const RED: Rgb = [0xFF, 0x00, 0x00];
const NAVY: Rgb = [0x00, 0x00, 0x80];
const INK: Rgb = [0x00, 0x00, 0x00];
const DIM: Rgb = [0x50, 0x50, 0x50];

/// How one cell looks.
#[derive(Clone, Copy)]
enum Cell {
    Hidden,
    Flag,
    /// Opened, with this many mines around it (0: empty).
    Open(u8),
}

/// The row of cells under the title.
const STRIP: [Cell; 7] = [
    Cell::Hidden,
    Cell::Open(1),
    Cell::Open(2),
    Cell::Flag,
    Cell::Open(3),
    Cell::Open(1),
    Cell::Hidden,
];

/// The hero art's board: 32 × 18 cells fill 16:9 exactly.
const HERO_COLS: usize = 32;
const HERO_ROWS: usize = 18;

/// The game's number colours (`theme::number_color`).
fn number_color(mines: u8) -> Rgb {
    match mines {
        1 => BLUE,
        2 => GREEN,
        3 => RED,
        4 => NAVY,
        5 => [0x80, 0x00, 0x00],
        6 => [0x00, 0x80, 0x80],
        7 => INK,
        _ => DARK,
    }
}

const TAGLINE: &str = "No ads · No account · Plays offline";

fn main() {
    let folder = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example store_art -- <output folder>");
    let folder = Path::new(&folder);
    fs::create_dir_all(folder).expect("create the output folder");
    let font = Font::from_bytes(
        include_bytes!("../assets/fonts/Roboto-Bold.ttf") as &[u8],
        FontSettings::default(),
    )
    .expect("Roboto Bold");

    for (width, height) in [(1440, 2160), (720, 1080)] {
        save(
            folder,
            &format!("PosterArt-{width}x{height}.png"),
            &poster(width, height, &font),
        );
    }
    for size in [2160, 1080] {
        save(folder, &format!("BoxArt-{size}x{size}.png"), &box_art(size, &font));
    }
    for (width, height) in [(3840, 2160), (1920, 1080)] {
        save(
            folder,
            &format!("HeroArt-{width}x{height}.png"),
            &hero(width, height, &font),
        );
    }
    for size in [300, 150, 71] {
        let tile = Canvas {
            width: size,
            height: size,
            pixels: app_icon::app_icon_rgba(size),
        };
        save(folder, &format!("AppTile-{size}x{size}.png"), &tile);
    }
}

/// 9:16, designed at 1440 × 2160: the icon, the name on two lines, a row of cells, the tagline.
fn poster(width: u32, height: u32, font: &Font) -> Canvas {
    let s = width as f32 / 1440.0;
    let mut c = Canvas::new(width, height, FACE);
    c.bevel(0.0, 0.0, width as f32, height as f32, 24.0 * s, true);

    c.icon(720.0 * s, 230.0 * s, 860.0 * s);
    c.text_fitted(font, "Classic", 150.0 * s, 720.0 * s, 1335.0 * s, 1200.0 * s, NAVY);
    c.text_fitted(font, "Mine Sweeper", 210.0 * s, 720.0 * s, 1560.0 * s, 1240.0 * s, INK);
    c.strip(font, 720.0 * s, 1680.0 * s, 160.0 * s);
    c.text_fitted(font, TAGLINE, 64.0 * s, 720.0 * s, 2010.0 * s, 1240.0 * s, DIM);
    c
}

/// 1:1, designed at 2160 × 2160: the same parts, the name on one line.
fn box_art(size: u32, font: &Font) -> Canvas {
    let s = size as f32 / 2160.0;
    let mut c = Canvas::new(size, size, FACE);
    c.bevel(0.0, 0.0, size as f32, size as f32, 32.0 * s, true);

    c.icon(1080.0 * s, 200.0 * s, 900.0 * s);
    c.text_fitted(
        font,
        "Classic Mine Sweeper",
        230.0 * s,
        1080.0 * s,
        1420.0 * s,
        1900.0 * s,
        INK,
    );
    c.strip(font, 1080.0 * s, 1560.0 * s, 200.0 * s);
    c.text_fitted(font, TAGLINE, 96.0 * s, 1080.0 * s, 1990.0 * s, 1900.0 * s, DIM);
    c
}

fn save(folder: &Path, name: &str, canvas: &Canvas) {
    let path = folder.join(name);
    let file = File::create(&path).expect("create the PNG file");
    let mut encoder = png::Encoder::new(BufWriter::new(file), canvas.width, canvas.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .and_then(|mut writer| writer.write_image_data(&canvas.pixels))
        .expect("write the PNG file");
    println!("{}", path.display());
}

/// An RGBA image being drawn on.
struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Canvas {
    fn new(width: u32, height: u32, color: Rgb) -> Self {
        let pixels = [color[0], color[1], color[2], 0xFF].repeat((width * height) as usize);
        Self { width, height, pixels }
    }

    /// Mixes `color` into one pixel; `alpha` 255 paints over it, 0 leaves it.
    fn blend(&mut self, x: i32, y: i32, color: Rgb, alpha: u8) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let i = ((y as u32 * self.width + x as u32) * 4) as usize;
        let a = u32::from(alpha);
        for (channel, &value) in color.iter().enumerate() {
            let old = u32::from(self.pixels[i + channel]);
            self.pixels[i + channel] = ((u32::from(value) * a + old * (255 - a)) / 255) as u8;
        }
    }

    fn fill(&mut self, x: f32, y: f32, w: f32, h: f32, color: Rgb) {
        for py in y.round() as i32..(y + h).round() as i32 {
            for px in x.round() as i32..(x + w).round() as i32 {
                self.blend(px, py, color, 0xFF);
            }
        }
    }

    /// A raised (light top-left, dark bottom-right) or sunken edge `b` wide around a rectangle,
    /// with the classic diagonal where the two colours meet.
    fn bevel(&mut self, x: f32, y: f32, w: f32, h: f32, b: f32, raised: bool) {
        let (first, second) = if raised { (LIGHT, DARK) } else { (DARK, LIGHT) };
        let (w, h, b) = (w.round() as i32, h.round() as i32, b.round() as i32);
        for dy in 0..h {
            for dx in 0..w {
                let top_left = (dy < b && dx < w - dy) || (dx < b && dy < h - dx);
                let bottom_right = dx >= w - b || dy >= h - b;
                let color = if top_left {
                    first
                } else if bottom_right {
                    second
                } else {
                    continue;
                };
                self.blend(x.round() as i32 + dx, y.round() as i32 + dy, color, 0xFF);
            }
        }
    }

    /// The game icon, `size` px, its top edge centred at (`center_x`, `top`).
    fn icon(&mut self, center_x: f32, top: f32, size: f32) {
        let size = size.round() as u32;
        let rgba = app_icon::app_icon_rgba(size);
        let left = (center_x - size as f32 / 2.0).round() as i32;
        for y in 0..size {
            for x in 0..size {
                let i = ((y * size + x) * 4) as usize;
                let color = [rgba[i], rgba[i + 1], rgba[i + 2]];
                self.blend(left + x as i32, top.round() as i32 + y as i32, color, rgba[i + 3]);
            }
        }
    }

    /// One line of text centred on `center_x`, standing on `baseline`; shrunk if it would be
    /// wider than `max_width`.
    #[allow(clippy::too_many_arguments)]
    fn text_fitted(
        &mut self,
        font: &Font,
        text: &str,
        px: f32,
        center_x: f32,
        baseline: f32,
        max_width: f32,
        color: Rgb,
    ) {
        let px = px.min(px * max_width / text_width(font, text, px));
        let mut x = center_x - text_width(font, text, px) / 2.0;
        let mut previous = None;
        for c in text.chars() {
            if let Some(p) = previous {
                x += font.horizontal_kern(p, c, px).unwrap_or(0.0);
            }
            let (m, coverage) = font.rasterize(c, px);
            let left = (x + m.xmin as f32).round() as i32;
            let top = (baseline - m.height as f32 - m.ymin as f32).round() as i32;
            for row in 0..m.height {
                for col in 0..m.width {
                    self.blend(
                        left + col as i32,
                        top + row as i32,
                        color,
                        coverage[row * m.width + col],
                    );
                }
            }
            x += m.advance_width;
            previous = Some(c);
        }
    }

    /// The row of cells, `cell` px each, centred on `center_x`.
    fn strip(&mut self, font: &Font, center_x: f32, top: f32, cell: f32) {
        let left = center_x - cell * STRIP.len() as f32 / 2.0;
        let bevel = cell / 8.0;
        // The sunken border around the row, like the board's.
        self.bevel(
            left - bevel,
            top - bevel,
            cell * STRIP.len() as f32 + 2.0 * bevel,
            cell + 2.0 * bevel,
            bevel,
            false,
        );
        for (i, &kind) in STRIP.iter().enumerate() {
            self.cell(font, left + i as f32 * cell, top, cell, kind);
        }
    }

    /// One cell, `size` px, the way the game draws it: raised while hidden, flat with a grey
    /// grid line on its top and left once open.
    fn cell(&mut self, font: &Font, x: f32, y: f32, size: f32, kind: Cell) {
        let label_baseline = y + size * 0.78;
        let label_px = size * 0.72;
        self.fill(x, y, size, size, FACE);
        match kind {
            Cell::Hidden | Cell::Flag => {
                self.bevel(x, y, size, size, size / 8.0, true);
                if let Cell::Flag = kind {
                    self.text_fitted(font, "F", label_px, x + size / 2.0, label_baseline, size, RED);
                }
            }
            Cell::Open(mines) => {
                let line = (size / 24.0).max(1.0);
                self.fill(x, y, size, line, DARK);
                self.fill(x, y, line, size, DARK);
                if mines > 0 {
                    let text = mines.to_string();
                    let color = number_color(mines);
                    self.text_fitted(font, &text, label_px, x + size / 2.0, label_baseline, size, color);
                }
            }
        }
    }
}

/// 16:9 "super hero art": a game in progress filling the whole picture. The Store puts the
/// app's name and buttons over its left side, so the opened part is on the right, and the
/// rules forbid the title in it.
fn hero(width: u32, height: u32, font: &Font) -> Canvas {
    let mut c = Canvas::new(width, height, FACE);
    let size = width as f32 / HERO_COLS as f32;
    for (index, kind) in hero_board().into_iter().enumerate() {
        let (row, col) = (index / HERO_COLS, index % HERO_COLS);
        c.cell(font, col as f32 * size, row as f32 * size, size, kind);
    }
    c
}

/// A believable game in progress: random mines, an opened area right of centre (grown
/// like the game grows it, through empty cells), and most mines around it flagged.
fn hero_board() -> Vec<Cell> {
    let index = |row: usize, col: usize| row * HERO_COLS + col;
    let neighbours = |row: usize, col: usize| {
        let mut around = Vec::new();
        for r in row.saturating_sub(1)..=(row + 1).min(HERO_ROWS - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(HERO_COLS - 1) {
                if (r, c) != (row, col) {
                    around.push((r, c));
                }
            }
        }
        around
    };

    // A fixed seed, so the picture is the same every time (xorshift: tiny, good enough here).
    let mut seed: u64 = 135_774;
    let mut random = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mines: Vec<bool> = (0..HERO_ROWS * HERO_COLS).map(|_| random() % 100 < 17).collect();
    let count = |row: usize, col: usize| {
        neighbours(row, col)
            .into_iter()
            .filter(|&(r, c)| mines[index(r, c)])
            .count() as u8
    };

    // Open every safe cell inside an ellipse, then let empty cells open their neighbours.
    let mut open = vec![false; mines.len()];
    for row in 0..HERO_ROWS {
        for col in 0..HERO_COLS {
            let dx = (col as f32 - 21.0) / 10.5;
            let dy = (row as f32 - 8.5) / 7.0;
            open[index(row, col)] = dx * dx + dy * dy <= 1.0 && !mines[index(row, col)];
        }
    }
    let mut grew = true;
    while grew {
        grew = false;
        for row in 0..HERO_ROWS {
            for col in 0..HERO_COLS {
                if open[index(row, col)] && count(row, col) == 0 {
                    for (r, c) in neighbours(row, col) {
                        if !open[index(r, c)] && !mines[index(r, c)] {
                            open[index(r, c)] = true;
                            grew = true;
                        }
                    }
                }
            }
        }
    }

    (0..mines.len())
        .map(|i| {
            let (row, col) = (i / HERO_COLS, i % HERO_COLS);
            let next_to_open = neighbours(row, col).into_iter().any(|(r, c)| open[index(r, c)]);
            if open[i] {
                Cell::Open(count(row, col))
            } else if mines[i] && next_to_open && (row * 7 + col * 3) % 5 != 0 {
                Cell::Flag // most of them: a few are left for the player to find
            } else {
                Cell::Hidden
            }
        })
        .collect()
}

fn text_width(font: &Font, text: &str, px: f32) -> f32 {
    let mut width = 0.0;
    let mut previous = None;
    for c in text.chars() {
        if let Some(p) = previous {
            width += font.horizontal_kern(p, c, px).unwrap_or(0.0);
        }
        width += font.metrics(c, px).advance_width;
        previous = Some(c);
    }
    width
}
