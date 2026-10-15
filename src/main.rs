mod logic;
mod ui;
use macroquad::prelude::*;

use crate::{
    logic::{Difficulty, Game},
    ui::{
        BoardLayout, ClickTracker, Face, Fonts, HeaderLayout, draw_board, draw_header, face_clicked, handle_mouse,
        pressing_board, theme,
    },
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
    let mut game = Game::new(DIFFICULTY);
    let mut tracker = ClickTracker::default();

    let layout = layout_for(DIFFICULTY);
    let header = HeaderLayout::above(layout.rect());
    let fonts = Fonts::load();

    loop {
        // The tracker must see every button event, including clicks on the face,
        // or it would think a button is still held down.
        handle_mouse(&mut game, &layout, &mut tracker);
        if face_clicked(&header) {
            game = Game::new(DIFFICULTY);
        }
        let face = Face::for_game(game.status(), pressing_board(&layout));

        clear_background(theme::BACKGROUND);
        draw_header(&game, &header, face, &fonts.counter);
        draw_board(&game, &layout, &fonts.cell);
        next_frame().await
    }
}
