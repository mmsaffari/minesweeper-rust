mod app;
mod logic;
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
    let mut app = App::new(Difficulty::default());
    let fonts = Fonts::load();

    loop {
        if let Some(size) = app.update() {
            request_new_screen_size(size.x, size.y);
        }
        app.draw(&fonts);
        next_frame().await
    }
}
