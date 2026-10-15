mod logic;
mod ui;
use macroquad::prelude::*;

use crate::{
    logic::{Difficulty, Game, Position},
    ui::{BoardLayout, Fonts, draw_board, theme},
};

const DIFFICULTY: Difficulty = Difficulty::Beginner;

fn layout_for(difficulty: Difficulty) -> BoardLayout {
    let props = difficulty.board_properties();
    BoardLayout::new(
        vec2(theme::SIDE_MARGIN, theme::TOP_MARGIN),
        theme::CELL_SIZE,
        props.rows,
        props.cols,
    )
}

fn window_conf() -> Conf {
    let board = layout_for(DIFFICULTY).size();
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: (theme::SIDE_MARGIN + board.x + theme::SIDE_MARGIN) as i32,
        window_height: (theme::TOP_MARGIN + board.y + theme::BOTTOM_MARGIN) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::with_seed(DIFFICULTY, 1);
    game.reveal(Position::new(1, 1));

    let layout = layout_for(DIFFICULTY);
    let fonts = Fonts::load();

    loop {
        clear_background(theme::BACKGROUND);
        draw_board(&game, &layout, &fonts.cell);
        next_frame().await
    }
}
