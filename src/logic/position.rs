#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
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
}
