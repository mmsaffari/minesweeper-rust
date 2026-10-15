//! Runs before the game is compiled (Cargo does this automatically for a `build.rs`).
//!
//! On Windows it embeds a resource into `minesweeper.exe`: the icon Explorer and the taskbar
//! show, plus version details from Cargo.toml (right-click → Properties → Details).
//! On other systems it does nothing; they have no such thing as an icon inside a program.

use std::env;
use std::fs;
use std::path::PathBuf;

// The same drawing the window icon uses, so the two icons can never differ.
#[path = "src/ui/app_icon.rs"]
mod app_icon;

/// The sizes Windows picks from: 16 (lists), 24 and 32 (taskbar, small views), 48–128 (big views).
const ICO_SIZES: [u32; 6] = [16, 24, 32, 48, 64, 128];

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/ui/app_icon.rs");

    // The *target* system, not the one building it: cross-compiling from Linux to Windows counts.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Cargo always sets OUT_DIR"));
    let ico_path = out_dir.join("minesweeper.ico");
    fs::write(&ico_path, encode_ico(&ICO_SIZES)).expect("write the .ico file");

    // The version numbers come from Cargo.toml automatically; these are the human-readable parts.
    winresource::WindowsResource::new()
        .set_icon(ico_path.to_str().expect("OUT_DIR is valid UTF-8"))
        .set("ProductName", "Minesweeper")
        .set("FileDescription", "Minesweeper")
        .set("LegalCopyright", "© 2026 Mahdi Saffari. MIT OR Apache-2.0.")
        .compile()
        .expect("embed the icon and version details in the .exe");
}

/// An `.ico` file: a small directory listing the images, then each image.
fn encode_ico(sizes: &[u32]) -> Vec<u8> {
    let images: Vec<Vec<u8>> = sizes.iter().map(|&size| bmp_image(size)).collect();

    let mut ico = Vec::new();
    for value in [0u16, 1, sizes.len() as u16] {
        ico.extend_from_slice(&value.to_le_bytes()); // reserved, type (1 = icon), image count
    }

    let mut offset = 6 + 16 * sizes.len() as u32; // the images start after the directory
    for (&size, image) in sizes.iter().zip(&images) {
        let side = if size >= 256 { 0 } else { size as u8 }; // 0 means 256
        ico.extend_from_slice(&[side, side, 0, 0]); // width, height, palette size, reserved
        ico.extend_from_slice(&1u16.to_le_bytes()); // colour planes
        ico.extend_from_slice(&32u16.to_le_bytes()); // bits per pixel
        ico.extend_from_slice(&(image.len() as u32).to_le_bytes());
        ico.extend_from_slice(&offset.to_le_bytes());
        offset += image.len() as u32;
    }

    for image in images {
        ico.extend(image);
    }
    ico
}

/// One image inside the `.ico`: a BMP header, the pixels as blue-green-red-alpha from the
/// *bottom* row up (that's how BMP stores them), then a 1-bit mask that old Windows versions
/// use for transparency. Ours is all zeros: the alpha channel already says it all.
fn bmp_image(size: u32) -> Vec<u8> {
    let rgba = app_icon::app_icon_rgba(size);
    let mask_row_bytes = size.div_ceil(32) * 4; // 1 bit per pixel, each row padded to 4 bytes

    let mut bmp = Vec::new();
    // BITMAPINFOHEADER (40 bytes). The height counts twice: the pixels plus the mask.
    for value in [40u32, size, size * 2] {
        bmp.extend_from_slice(&value.to_le_bytes()); // header size, width, height
    }
    bmp.extend_from_slice(&1u16.to_le_bytes()); // colour planes
    bmp.extend_from_slice(&32u16.to_le_bytes()); // bits per pixel
    let image_bytes = size * size * 4 + mask_row_bytes * size;
    for value in [0u32, image_bytes, 0, 0, 0, 0] {
        bmp.extend_from_slice(&value.to_le_bytes()); // no compression, size, unused fields
    }

    for row in (0..size).rev() {
        for col in 0..size {
            let i = ((row * size + col) * 4) as usize;
            bmp.extend_from_slice(&[rgba[i + 2], rgba[i + 1], rgba[i], rgba[i + 3]]); // RGBA → BGRA
        }
    }
    bmp.resize(bmp.len() + (mask_row_bytes * size) as usize, 0);
    bmp
}
