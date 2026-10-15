use super::Press;
use crate::logic::{CellState, Game, GameStatus, Position};

/// The cells that should look pushed in this frame, like in the original: the cell under
/// a held left button, or for a chord that cell and its neighbours. Only cells a click
/// could still open (hidden or `?`) are pushed in; flags and opened cells stay as they are.
pub fn pressed_cells(game: &Game, under_mouse: Option<Position>, press: Press) -> Vec<Position> {
    let finished = matches!(game.status(), GameStatus::Won | GameStatus::Lost);
    let Some(pos) = under_mouse else {
        return Vec::new();
    };
    if finished {
        return Vec::new();
    }

    let candidates = match press {
        Press::None => return Vec::new(),
        Press::Single => vec![pos],
        Press::Chord => {
            let mut cells = game.board().neighbours(pos);
            cells.push(pos);
            cells
        }
    };

    candidates
        .into_iter()
        .filter(|&p| {
            game.board()
                .cell(p)
                .is_some_and(|cell| matches!(cell.state(), CellState::Hidden | CellState::Questioned))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::Difficulty;
    use std::collections::HashSet;

    fn p(row: usize, col: usize) -> Position {
        Position::new(row, col)
    }

    fn as_set(cells: Vec<Position>) -> HashSet<Position> {
        cells.into_iter().collect()
    }

    /// A fresh Beginner game: every cell hidden, no mines yet.
    fn fresh() -> Game {
        Game::new(Difficulty::Beginner)
    }

    #[test]
    fn nothing_is_pressed_without_a_press_or_off_the_board() {
        let game = fresh();
        assert!(pressed_cells(&game, Some(p(4, 4)), Press::None).is_empty());
        assert!(pressed_cells(&game, None, Press::Single).is_empty());
        assert!(pressed_cells(&game, None, Press::Chord).is_empty());
    }

    #[test]
    fn a_single_press_pushes_in_only_the_cell_under_the_mouse() {
        assert_eq!(pressed_cells(&fresh(), Some(p(4, 4)), Press::Single), [p(4, 4)]);
    }

    #[test]
    fn a_chord_pushes_in_the_cell_and_its_neighbours() {
        let pressed = as_set(pressed_cells(&fresh(), Some(p(0, 0)), Press::Chord));
        assert_eq!(
            pressed,
            HashSet::from([p(0, 0), p(0, 1), p(1, 0), p(1, 1)]),
            "a corner has 3 neighbours"
        );
    }

    #[test]
    fn flags_are_never_pushed_in_but_question_marks_are() {
        let mut game = fresh();
        game.toggle_mark(p(0, 1)); // flag
        game.toggle_mark(p(1, 0));
        game.toggle_mark(p(1, 0)); // question mark

        let pressed = as_set(pressed_cells(&game, Some(p(0, 0)), Press::Chord));
        assert_eq!(pressed, HashSet::from([p(0, 0), p(1, 0), p(1, 1)]));
        assert!(
            pressed_cells(&game, Some(p(0, 1)), Press::Single).is_empty(),
            "pressing a flag"
        );
    }

    #[test]
    fn opened_cells_are_never_pushed_in() {
        let mut game = fresh();
        game.reveal(p(4, 4)); // the first click is always safe, and opens at least this cell
        assert!(pressed_cells(&game, Some(p(4, 4)), Press::Single).is_empty());
        assert!(!pressed_cells(&game, Some(p(4, 4)), Press::Chord).contains(&p(4, 4)));
    }
}
