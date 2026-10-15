/// A cell's place on the board. `Ord` sorts row by row, top-left first (rows compare before
/// columns because `row` is the first field), so sorted positions read like a page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// The up-to-8 surrounding positions that are on a `rows` × `cols` board.
    pub fn neighbours(self, rows: usize, cols: usize) -> Vec<Position> {
        let mut result = Vec::with_capacity(8);
        for dr in [-1, 0, 1] {
            for dc in [-1, 0, 1] {
                if dr == 0 && dc == 0 {
                    continue;
                }
                // `checked_add_signed` gives `None` below 0 (the top/left edges);
                // the comparison rules out the bottom/right edges. No casts needed.
                let (Some(row), Some(col)) = (self.row.checked_add_signed(dr), self.col.checked_add_signed(dc)) else {
                    continue;
                };
                if row < rows && col < cols {
                    result.push(Position::new(row, col));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_row_and_col() {
        let pos = Position::new(2, 3);
        assert_eq!(pos.row, 2);
        assert_eq!(pos.col, 3);
    }

    #[test]
    fn sorts_row_by_row() {
        let mut positions = [Position::new(1, 0), Position::new(0, 5), Position::new(0, 1)];
        positions.sort();
        assert_eq!(
            positions,
            [Position::new(0, 1), Position::new(0, 5), Position::new(1, 0)]
        );
    }

    // `neighbours` is tested thoroughly through `Board::neighbours`, which uses it
    // (including the brute-force property test over a whole board).
}
