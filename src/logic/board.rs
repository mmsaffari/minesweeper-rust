use super::BoardProperties;
use super::Cell;
use super::CellContent;
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

    /// Read-only, for the UI (which lives outside `logic`, so it can't see the fields).
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn cell(&self, pos: Position) -> Option<&Cell> {
        self.cells.get(pos.row)?.get(pos.col)
    }

    pub fn cell_mut(&mut self, pos: Position) -> Option<&mut Cell> {
        self.cells.get_mut(pos.row)?.get_mut(pos.col)
    }

    pub fn neighbours(&self, pos: Position) -> Vec<Position> {
        pos.neighbours(self.rows, self.cols)
    }

    /// Call once, on a fresh board. Panics if a position is off the board.
    pub fn place_mines(&mut self, mines: &[Position]) {
        // Pass 1: put the mines down. Listing a position twice is harmless.
        for pos in mines {
            self.cells[pos.row][pos.col].content = CellContent::Mine;
        }

        // Pass 2: every empty cell counts the mines around it. This counts what is actually
        // on the board, not what was in the list, so duplicates can't be counted twice.
        for row in 0..self.rows {
            for col in 0..self.cols {
                let count = self
                    .neighbours(Position::new(row, col))
                    .into_iter()
                    .filter(|n| self.cells[n.row][n.col].is_mine())
                    .count();

                // Only an `Empty` cell has a count to set; for a `Mine` there is no field to write.
                if let CellContent::Empty { adjacent_mines } = &mut self.cells[row][col].content {
                    *adjacent_mines = u8::try_from(count).expect("a cell has at most 8 neighbours");
                }
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

    /// What's under every cell, one string per row: `*` for a mine, otherwise the count
    /// (including 0), so one assert can compare a whole board.
    fn contents(board: &Board) -> Vec<String> {
        board
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell.content {
                        CellContent::Mine => '*',
                        CellContent::Empty { adjacent_mines } => char::from(b'0' + adjacent_mines),
                    })
                    .collect()
            })
            .collect()
    }

    fn mine_positions(board: &Board) -> HashSet<Position> {
        let mut mines = HashSet::new();
        for (row, cells) in board.cells.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                if cell.is_mine() {
                    mines.insert(p(row, col));
                }
            }
        }
        mines
    }

    // --- new ---

    #[test]
    fn rows_and_cols_match_the_properties() {
        let board = board(4, 7);
        assert_eq!((board.rows(), board.cols()), (4, 7));
    }

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

    // --- cell ---

    #[test]
    fn cell_is_found_inside_and_not_outside() {
        let board = example_board();
        let (rows, cols) = (EXAMPLE_ROWS, EXAMPLE_COLS);

        for pos in [p(0, 0), p(1, 1), p(rows / 2, cols / 2), p(rows - 1, cols - 1)] {
            assert!(board.cell(pos).is_some(), "{pos:?} should be on the board");
        }
        for pos in [
            p(0, cols),
            p(rows, 0),
            p(rows / 2, cols),
            p(rows, cols / 2),
            p(rows, cols),
            p(rows + 1, cols + 1),
        ] {
            assert!(board.cell(pos).is_none(), "{pos:?} should NOT be on the board");
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
        assert_eq!(contents(&board), ["*10", "121", "01*"]);
    }

    /// The point of `CellContent`: a mine is just a mine, it has no count of its own.
    #[test]
    fn mines_next_to_each_other_have_no_count_of_their_own() {
        let mut board = board(3, 3);
        board.place_mines(&[p(0, 0), p(0, 1)]);
        assert_eq!(contents(&board), ["**1", "221", "000"]);
    }

    #[test]
    fn place_mines_works_on_non_square_board() {
        let mut board = board(2, 4);
        board.place_mines(&[p(1, 3)]);
        assert_eq!(contents(&board), ["0011", "001*"]);
        assert_eq!(mine_positions(&board), HashSet::from([p(1, 3)]));
    }

    #[test]
    fn place_mines_with_no_mines_leaves_board_empty() {
        let mut board = example_board();
        board.place_mines(&[]);
        assert!(mine_positions(&board).is_empty());
        assert!(all_cells(&board).all(|cell| cell.content == CellContent::Empty { adjacent_mines: 0 }));
    }

    #[test]
    fn duplicate_positions_count_as_one_mine() {
        let mut board = board(3, 3);
        board.place_mines(&[p(1, 1), p(1, 1)]);
        assert_eq!(contents(&board), ["111", "1*1", "111"]);
    }
}
