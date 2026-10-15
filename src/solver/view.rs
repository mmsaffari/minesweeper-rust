use std::fmt;

use super::Seen;
use crate::logic::{CellContent, CellState, Game, Position};

/// The board as the player sees it: what's opened, flagged or marked, and the numbers.
/// Where the hidden mines are is not in here at all.
///
/// Its text form (`Display`) is the board export: one line per row, using the characters
/// of `Seen::to_char`, e.g.
///
/// ```text
/// ..1#
/// .12#
/// .1F#
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    cells: Vec<Vec<Seen>>,
}

impl View {
    pub fn from_game(game: &Game) -> Self {
        let board = game.board();
        let cells = (0..board.rows())
            .map(|row| {
                (0..board.cols())
                    .map(|col| {
                        let cell = board.cell(Position::new(row, col)).expect("inside the board");
                        match (cell.state(), cell.content()) {
                            (CellState::Hidden, _) => Seen::Hidden,
                            (CellState::Questioned, _) => Seen::Questioned,
                            (CellState::Flagged, _) => Seen::Flagged,
                            (CellState::Revealed, CellContent::Mine) => Seen::Mine,
                            (CellState::Revealed, CellContent::Empty { adjacent_mines }) => {
                                Seen::Number(adjacent_mines)
                            }
                        }
                    })
                    .collect()
            })
            .collect();
        Self { cells }
    }
}

/// The text form: one line per row, no trailing newline.
impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            for seen in row {
                write!(f, "{}", seen.to_char())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::logic::Difficulty;

    /// Reads the text form back into a `View`, so tests can describe a board as a picture.
    /// The game never needs this (it only exports), so it lives with the tests.
    /// Spaces around rows and blank lines are ignored.
    pub(crate) fn parse(text: &str) -> View {
        let cells: Vec<Vec<Seen>> = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(from_char).collect())
            .collect();
        assert!(!cells.is_empty(), "the board is empty");
        let cols = cells[0].len();
        for (row, cells) in cells.iter().enumerate() {
            assert_eq!(cells.len(), cols, "row {row} isn't as long as the first one");
        }
        View { cells }
    }

    fn from_char(c: char) -> Seen {
        match c {
            '#' => Seen::Hidden,
            '?' => Seen::Questioned,
            'F' => Seen::Flagged,
            '.' => Seen::Number(0),
            '*' => Seen::Mine,
            '1'..='8' => Seen::Number(c as u8 - b'0'),
            _ => panic!("{c:?} isn't a board character"),
        }
    }

    #[test]
    fn a_new_game_shows_only_hidden_cells() {
        let view = View::from_game(&Game::new(Difficulty::Beginner));
        assert_eq!(view.to_string(), ["#########"; 9].join("\n"));
    }

    #[test]
    fn shows_marks_and_opened_cells_but_never_hidden_mines() {
        let mut game = Game::new(Difficulty::Beginner);
        game.toggle_mark(Position::new(0, 0)); // flag
        game.toggle_mark(Position::new(0, 1));
        game.toggle_mark(Position::new(0, 1)); // question mark
        game.reveal(Position::new(8, 8)); // the first click: safe, and the mines get placed

        let text = View::from_game(&game).to_string();
        assert!(text.starts_with("F?"), "{text}");
        assert!(!text.contains('*'), "a hidden mine must never show:\n{text}");
        assert_ne!(text.lines().nth(8).unwrap().chars().last(), Some('#'), "(8, 8) is open");
    }

    #[test]
    fn an_exported_board_reads_back_the_same() {
        let mut game = Game::new(Difficulty::Intermediate);
        game.reveal(Position::new(5, 5));
        game.toggle_mark(Position::new(0, 0));
        let view = View::from_game(&game);
        assert_eq!(parse(&view.to_string()), view);
    }

    #[test]
    fn parse_ignores_surrounding_whitespace() {
        assert_eq!(parse("\n  #1 \n  F. \n\n").to_string(), "#1\nF.");
    }
}
