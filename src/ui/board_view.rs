use macroquad::prelude::*;

use super::BoardLayout;
use super::bevel;
use super::mine_icon;
use super::theme;
use crate::logic::{Cell, CellContent, CellState, Game, GameStatus, Position};

/// Draws every cell of the board. Only reads the game; changing it is the input's job.
/// `pressed`: cells to show pushed in (see `pressed_cells`).
pub fn draw_board(game: &Game, layout: &BoardLayout, pressed: &[Position], font: &Font) {
    let board = game.board();
    let lost = game.status() == GameStatus::Lost;

    for row in 0..board.rows() {
        for col in 0..board.cols() {
            let pos = Position::new(row, col);
            if let Some(cell) = board.cell(pos) {
                draw_cell(cell, layout.cell_rect(pos), lost, pressed.contains(&pos), font);
            }
        }
    }

    bevel::draw_sunken_border(layout.rect(), theme::BEVEL);
}

fn draw_cell(cell: &Cell, rect: Rect, lost: bool, pressed: bool, font: &Font) {
    // Arms are tried top to bottom, so the special "game lost" cases (with `if` guards)
    // must come before the normal arms for the same state.
    match cell.state() {
        CellState::Revealed => draw_revealed(cell, rect, font),

        // After losing, show every mine the player hadn't found...
        CellState::Hidden | CellState::Questioned if lost && cell.is_mine() => {
            draw_flat(rect, theme::BACKGROUND);
            draw_mine(rect);
        }
        // ...and cross out the flags that were wrong.
        CellState::Flagged if lost && !cell.is_mine() => {
            draw_flat(rect, theme::BACKGROUND);
            draw_mine(rect);
            draw_cross(rect);
        }

        // Held down under the mouse: pushed in, i.e. flat like an opened empty cell.
        CellState::Hidden if pressed => draw_flat(rect, theme::BACKGROUND),
        CellState::Questioned if pressed => {
            draw_flat(rect, theme::BACKGROUND);
            draw_centered_text("?", rect, theme::QUESTION_MARK, font);
        }

        CellState::Hidden => draw_raised(rect),
        CellState::Flagged => {
            draw_raised(rect);
            draw_centered_text("F", rect, theme::FLAG, font);
        }
        CellState::Questioned => {
            draw_raised(rect);
            draw_centered_text("?", rect, theme::QUESTION_MARK, font);
        }
    }
}

fn draw_revealed(cell: &Cell, rect: Rect, font: &Font) {
    match cell.content() {
        CellContent::Mine => {
            // The only revealed mine is the one that was clicked.
            draw_flat(rect, theme::EXPLODED);
            draw_mine(rect);
        }
        CellContent::Empty { adjacent_mines: 0 } => draw_flat(rect, theme::BACKGROUND),
        CellContent::Empty { adjacent_mines: n } => {
            draw_flat(rect, theme::BACKGROUND);
            draw_centered_text(&n.to_string(), rect, theme::number_color(n), font);
        }
    }
}

/// A hidden cell: raised like a button.
fn draw_raised(rect: Rect) {
    bevel::draw_raised(rect, theme::BEVEL, theme::BACKGROUND);
}

/// An opened cell: flat, with a thin grid line on its top and left.
/// The neighbour to the right/below draws the other two lines.
fn draw_flat(rect: Rect, fill: Color) {
    let Rect { x, y, w, h } = rect;
    draw_rectangle(x, y, w, h, fill);
    draw_rectangle(x, y, w, theme::GRID_LINE, theme::GRID); // top
    draw_rectangle(x, y, theme::GRID_LINE, h, theme::GRID); // left
}

/// A mine filling the cell.
fn draw_mine(rect: Rect) {
    mine_icon::draw_mine(rect.center(), rect.w * 0.25, 1.0);
}

/// The red X over a wrong flag after losing.
fn draw_cross(rect: Rect) {
    let inset = rect.w * 0.2;
    let (left, right) = (rect.x + inset, rect.x + rect.w - inset);
    let (top, bottom) = (rect.y + inset, rect.y + rect.h - inset);
    draw_line(left, top, right, bottom, 2.0, theme::EXPLODED);
    draw_line(left, bottom, right, top, 2.0, theme::EXPLODED);
}

/// `draw_text`'s y is the text's baseline (the line letters sit on), not its top,
/// so centring needs the size `measure_text` reports.
fn draw_centered_text(text: &str, rect: Rect, color: Color, font: &Font) {
    let size = measure_text(text, Some(font), theme::FONT_SIZE, 1.0);
    let x = rect.x + (rect.w - size.width) / 2.0;
    let y = rect.y + (rect.h - size.height) / 2.0 + size.offset_y;
    let params = TextParams {
        font: Some(font),
        font_size: theme::FONT_SIZE,
        color,
        ..Default::default()
    };
    draw_text_ex(text, x, y, params);
}
