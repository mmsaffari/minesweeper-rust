// In release builds on Windows, start as a window app, without a console window behind it.
// Debug builds keep the console, so `eprintln!` messages stay visible while developing.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod logic;
mod scores;
mod solver;
mod storage;
mod ui;
mod updates;

use std::time::Instant;

use macroquad::miniquad::conf::{Icon, Platform};
use macroquad::prelude::*;

use crate::app::App;
use crate::logic::Difficulty;
use crate::storage::Stored;
use crate::ui::{BoardLayout, Fonts, FramePacer, app_icon_rgba, window_size};

/// Plenty for Minesweeper, and a fraction of what a fast screen would otherwise get.
const FRAMES_PER_SECOND: u32 = 60;

/// Runs before `main`, so it can't see the `App`: it only needs the starting window size.
fn window_conf() -> Conf {
    let size = window_size(BoardLayout::for_difficulty(Difficulty::default()).rect());
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: size.x as i32,
        window_height: size.y as i32,
        window_resizable: false,
        icon: Some(window_icon()),
        platform: Platform {
            // No vsync: some drivers keep a CPU core busy waiting for the screen's next
            // refresh. `FramePacer` sets the pace instead, and sleeps while it waits.
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// The mine icon in the three sizes miniquad wants. `try_into` turns each `Vec` into the
/// fixed-size array the `Icon` fields need; it can only fail if the sizes didn't match.
fn window_icon() -> Icon {
    Icon {
        small: app_icon_rgba(16).try_into().expect("16 x 16 RGBA"),
        medium: app_icon_rgba(32).try_into().expect("32 x 32 RGBA"),
        big: app_icon_rgba(64).try_into().expect("64 x 64 RGBA"),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let best_times = Stored::load("best_times.json");
    let settings = Stored::load("settings.json");
    let mut app = App::new(Difficulty::default(), best_times, settings);
    app.start_update_check();
    let fonts = Fonts::load();
    let mut pacer = FramePacer::new(FRAMES_PER_SECOND, Instant::now());

    loop {
        if let Some(size) = app.update() {
            request_new_screen_size(size.x, size.y);
        }
        app.draw(&fonts);
        pacer.wait();
        next_frame().await
    }
}
