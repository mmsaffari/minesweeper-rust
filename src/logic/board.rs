use std::collections::HashSet;

use super::BoardProperties;
use super::Cell;
use super::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    rows: usize,
    cols: usize,
    pub(super) cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(properties: BoardProperties) -> Self {
        Self {
            rows: properties.rows,
            cols: properties.cols,
            cells: vec![vec![Cell::default(); properties.cols]; properties.rows],
        }
    }

    pub fn contains(&self, pos: Position) -> bool {
        self.cell(pos).is_some()
    }

    pub fn cell(&self, pos: Position) -> Option<&Cell> {
        self.cells.get(pos.row)?.get(pos.col)
    }

    pub fn cell_mut(&mut self, pos: Position) -> Option<&mut Cell> {
        self.cells.get_mut(pos.row)?.get_mut(pos.col)
    }

    pub fn neighbours(&self, pos: Position) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::with_capacity(8);
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if i == 0 && j == 0 {
                    continue;
                }
                let n_row = pos.row as isize + i;
                let n_col = pos.col as isize + j;
                if n_row >= 0 && n_row < self.rows as isize && n_col >= 0 && n_col < self.cols as isize {
                    result.push(Position::new(n_row as usize, n_col as usize));
                }
            }
        }
        result
    }

    /// Call once, on a fresh board. Panics if a position is off the board.
    pub fn place_mines(&mut self, mines: &[Position]) {
        let unique_mines = mines.iter().copied().collect::<HashSet<Position>>();
        for pos in unique_mines {
            self.cells[pos.row][pos.col].is_mine = true;
            for n_pos in self.neighbours(pos) {
                self.cells[n_pos.row][n_pos.col].adjacent_mines += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::CellState;
    use std::collections::HashSet;

    // --- helpers ---

    const EXAMPLE_ROWS: usize = 9;
    const EXAMPLE_COLS: usize = 9;

    fn example_board() -> Board {
        board(EXAMPLE_ROWS, EXAMPLE_COLS)
    }

    fn board(rows: usize, cols: usize) -> Board {
        Board::new(BoardProperties { rows, cols, mines: 0 })
    }

    /// Shorthand so expected-position lists stay readable.
    fn p(row: usize, col: usize) -> Position {
        Position::new(row, col)
    }

    fn all_cells(board: &Board) -> impl Iterator<Item = &Cell> {
        board.cells.iter().flatten()
    }

    /// Checks the count on the raw Vec (catches duplicates), then the contents as a set (order-independent).
    fn assert_neighbours(board: &Board, pos: Position, expected: &[Position]) {
        let actual = board.neighbours(pos);
        assert_eq!(
            actual.len(),
            expected.len(),
            "wrong neighbour count for {pos:?}: {actual:?}"
        );
        let actual: HashSet<Position> = actual.into_iter().collect();
        let expected: HashSet<Position> = expected.iter().copied().collect();
        assert_eq!(actual, expected, "wrong neighbours for {pos:?}");
    }

    /// The whole grid of `adjacent_mines`, so one assert can compare a full board.
    fn adjacent_counts(board: &Board) -> Vec<Vec<u8>> {
        board
            .cells
            .iter()
            .map(|row| row.iter().map(|cell| cell.adjacent_mines).collect())
            .collect()
    }

    fn mine_positions(board: &Board) -> HashSet<Position> {
        let mut mines = HashSet::new();
        for (row, cells) in board.cells.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                if cell.is_mine {
                    mines.insert(p(row, col));
                }
            }
        }
        mines
    }

    // --- new ---

    #[test]
    fn new_board_has_rows_times_cols_cells() {
        let board = board(4, 7);
        assert_eq!(all_cells(&board).count(), 4 * 7);
    }

    #[test]
    fn new_board_cells_are_hidden_and_mine_free() {
        let board = example_board();
        assert!(
            all_cells(&board).all(|cell| cell.state == CellState::Hidden),
            "every cell should start hidden"
        );
        assert!(mine_positions(&board).is_empty(), "a new board should have no mines");
    }

    // --- contains / cell ---

    #[test]
    fn contains_is_true_inside_and_false_outside() {
        let board = example_board();
        let (rows, cols) = (EXAMPLE_ROWS, EXAMPLE_COLS);

        for pos in [p(0, 0), p(1, 1), p(rows / 2, cols / 2), p(rows - 1, cols - 1)] {
            assert!(board.contains(pos), "{pos:?} should be on the board");
        }
        for pos in [
            p(0, cols),
            p(rows, 0),
            p(rows / 2, cols),
            p(rows, cols / 2),
            p(rows, cols),
            p(rows + 1, cols + 1),
        ] {
            assert!(!board.contains(pos), "{pos:?} should NOT be on the board");
        }
    }

    #[test]
    fn cell_is_some_inside_and_none_outside() {
        let board = example_board();
        assert!(board.cell(p(EXAMPLE_ROWS / 2, EXAMPLE_COLS / 2)).is_some());
        assert!(board.cell(p(EXAMPLE_ROWS, EXAMPLE_COLS)).is_none());
    }

    // --- neighbours ---

    #[test]
    fn neighbours_of_corners() {
        let board = example_board();
        let (last_row, last_col) = (EXAMPLE_ROWS - 1, EXAMPLE_COLS - 1);
        assert_neighbours(&board, p(0, 0), &[p(0, 1), p(1, 0), p(1, 1)]);
        assert_neighbours(
            &board,
            p(0, last_col),
            &[p(0, last_col - 1), p(1, last_col - 1), p(1, last_col)],
        );
        assert_neighbours(
            &board,
            p(last_row, 0),
            &[p(last_row - 1, 0), p(last_row - 1, 1), p(last_row, 1)],
        );
        assert_neighbours(
            &board,
            p(last_row, last_col),
            &[
                p(last_row - 1, last_col - 1),
                p(last_row - 1, last_col),
                p(last_row, last_col - 1),
            ],
        );
    }

    #[test]
    fn neighbours_of_edge() {
        let board = example_board();
        assert_neighbours(&board, p(1, 0), &[p(0, 0), p(0, 1), p(1, 1), p(2, 0), p(2, 1)]);
    }

    #[test]
    fn neighbours_of_middle() {
        let board = example_board();
        assert_neighbours(
            &board,
            p(1, 1),
            &[p(0, 0), p(0, 1), p(0, 2), p(1, 0), p(1, 2), p(2, 0), p(2, 1), p(2, 2)],
        );
    }

    /// Property test: instead of hand-picked cases, check every cell against a brute-force answer.
    /// The board is deliberately NOT square, so a rows/cols mix-up can't hide.
    #[test]
    fn neighbours_match_brute_force_for_every_cell() {
        let (rows, cols) = (4, 7);
        let board = board(rows, cols);
        let all_positions: Vec<Position> = (0..rows).flat_map(|r| (0..cols).map(move |c| p(r, c))).collect();

        for &pos in &all_positions {
            let expected: Vec<Position> = all_positions
                .iter()
                .copied()
                .filter(|q| *q != pos && q.row.abs_diff(pos.row) <= 1 && q.col.abs_diff(pos.col) <= 1)
                .collect();
            assert_neighbours(&board, pos, &expected);
        }
    }

    // --- place_mines ---

    #[test]
    fn place_mines_marks_exactly_the_given_cells() {
        let mut board = board(3, 3);
        board.place_mines(&[p(0, 0), p(2, 2)]);
        assert_eq!(mine_positions(&board), HashSet::from([p(0, 0), p(2, 2)]));
    }

    #[test]
    fn place_mines_counts_adjacent_mines_on_every_cell() {
        let mut board = board(3, 3);
        board.place_mines(&[p(0, 0), p(2, 2)]);
        #[rustfmt::skip]
        let expected = vec![
            vec![0, 1, 0],
            vec![1, 2, 1],
            vec![0, 1, 0],
        ];
        assert_eq!(adjacent_counts(&board), expected);
    }

    #[test]
    fn mines_next_to_each_other_count_each_other() {
        let mut board = board(3, 3);
        board.place_mines(&[p(0, 0), p(0, 1)]);
        #[rustfmt::skip]
        let expected = vec![
            vec![1, 1, 1],
            vec![2, 2, 1],
            vec![0, 0, 0],
        ];
        assert_eq!(adjacent_counts(&board), expected);
    }

    #[test]
    fn place_mines_works_on_non_square_board() {
        let mut board = board(2, 4);
        board.place_mines(&[p(1, 3)]);
        #[rustfmt::skip]
        let expected = vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 0],
        ];
        assert_eq!(adjacent_counts(&board), expected);
        assert_eq!(mine_positions(&board), HashSet::from([p(1, 3)]));
    }

    #[test]
    fn place_mines_with_no_mines_leaves_board_empty() {
        let mut board = example_board();
        board.place_mines(&[]);
        assert!(mine_positions(&board).is_empty());
        assert!(all_cells(&board).all(|cell| cell.adjacent_mines == 0));
    }

    #[test]
    fn duplicate_positions_count_as_one_mine() {
        let mut board = board(3, 3);
        board.place_mines(&[p(1, 1), p(1, 1)]);
        #[rustfmt::skip]
        let expected = vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ];
        assert_eq!(adjacent_counts(&board), expected);
    }
}
