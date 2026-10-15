mod logic;
mod ui;
use macroquad::prelude::*;

use crate::{
    logic::{Difficulty, Game, Position},
    ui::{BoardLayout, draw_board, theme},
};

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
    let mut game = Game::with_seed(Difficulty::Beginner, 1);
    game.reveal(Position::new(1, 1));

    let layout = BoardLayout::new(
        vec2(12.0, 60.0),
        theme::CELL_SIZE,
        game.board().rows(),
        game.board().cols(),
    );

    loop {
        clear_background(theme::BACKGROUND);
        draw_board(&game, &layout);
        next_frame().await
    }
}
