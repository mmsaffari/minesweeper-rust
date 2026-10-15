//! Draws the logo files the Microsoft Store package needs, from the same drawing as the
//! window and `.exe` icons.
//!
//! `packaging/msix/build-msix.ps1` runs it:
//!
//! ```text
//! cargo run --example store_logos -- <output folder>
//! ```
//!
//! Windows picks a file by its name: `scale-200` is for a screen at 200 % zoom,
//! `targetsize-32` is an exact 32 px icon (taskbar, Start menu list), and `altform-unplated`
//! means "drawn without a coloured tile behind it". `MakePri` (in the script) indexes them.

use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

// Like build.rs: this file only uses `std`, so it can be included directly.
#[path = "../src/ui/app_icon.rs"]
mod app_icon;

/// One logo file: a `canvas` × `canvas` image with the icon, `icon` px wide, in its centre.
struct Logo {
    name: String,
    canvas: u32,
    icon: u32,
}

const SCALES: [u32; 3] = [100, 200, 400];
const TARGET_SIZES: [u32; 5] = [16, 24, 32, 48, 256];

/// Every file the manifest's logos need.
fn logos() -> Vec<Logo> {
    let mut logos = Vec::new();
    for scale in SCALES {
        let px = |base: u32| base * scale / 100;
        // Start menu and taskbar, and the logo in the Store: the icon fills them.
        logos.push(Logo {
            name: format!("Square44x44Logo.scale-{scale}.png"),
            canvas: px(44),
            icon: px(44),
        });
        logos.push(Logo {
            name: format!("StoreLogo.scale-{scale}.png"),
            canvas: px(50),
            icon: px(50),
        });
        // The Start menu tile: the icon in the middle, with room around it.
        logos.push(Logo {
            name: format!("Square150x150Logo.scale-{scale}.png"),
            canvas: px(150),
            icon: px(90),
        });
    }
    for size in TARGET_SIZES {
        for suffix in ["", "_altform-unplated"] {
            logos.push(Logo {
                name: format!("Square44x44Logo.targetsize-{size}{suffix}.png"),
                canvas: size,
                icon: size,
            });
        }
    }
    logos
}

fn main() {
    let folder = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example store_logos -- <output folder>");
    let folder = Path::new(&folder);
    fs::create_dir_all(folder).expect("create the output folder");

    for logo in logos() {
        let path = folder.join(&logo.name);
        write_png(&path, logo.canvas, &centered(logo.canvas, logo.icon));
        println!("{}", path.display());
    }
}

/// RGBA pixels of a transparent `canvas` × `canvas` square with the icon in its centre.
fn centered(canvas: u32, icon: u32) -> Vec<u8> {
    let rgba = app_icon::app_icon_rgba(icon);
    let mut pixels = vec![0; (canvas * canvas * 4) as usize];
    let margin = (canvas - icon) / 2;
    let row_bytes = (icon * 4) as usize;
    for row in 0..icon {
        let from = (row * icon * 4) as usize;
        let to = (((row + margin) * canvas + margin) * 4) as usize;
        pixels[to..to + row_bytes].copy_from_slice(&rgba[from..from + row_bytes]);
    }
    pixels
}

fn write_png(path: &Path, size: u32, rgba: &[u8]) {
    let file = File::create(path).expect("create the PNG file");
    let mut encoder = png::Encoder::new(BufWriter::new(file), size, size);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .and_then(|mut writer| writer.write_image_data(rgba))
        .expect("write the PNG file");
}
