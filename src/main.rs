// In release builds on Windows, start as a window app, without a console window behind it.
// Debug builds keep the console, so `eprintln!` messages stay visible while developing.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod logic;
mod scores;
mod ui;

use macroquad::prelude::*;

use crate::app::App;
use crate::logic::Difficulty;
use crate::ui::{BoardLayout, Fonts, window_size};

/// Runs before `main`, so it can't see the `App`: it only needs the starting window size.
fn window_conf() -> Conf {
    let size = window_size(BoardLayout::for_difficulty(Difficulty::default()).rect());
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: size.x as i32,
        window_height: size.y as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let best_times_path = scores::default_path();
    let best_times = best_times_path
        .as_deref()
        .map(scores::load_or_default)
        .unwrap_or_default();
    let mut app = App::new(Difficulty::default(), best_times, best_times_path);
    let fonts = Fonts::load();

    loop {
        if let Some(size) = app.update() {
            request_new_screen_size(size.x, size.y);
        }
        app.draw(&fonts);
        next_frame().await
    }
}
