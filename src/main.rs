mod logic;
use macroquad::prelude::*;
use std::time::Duration;

const FRAME_TIME: Duration = Duration::from_millis(33);

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: 300,
        window_height: 400,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        std::thread::sleep(FRAME_TIME);

        next_frame().await
    }
}
