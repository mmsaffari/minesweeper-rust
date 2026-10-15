/// What's under a cell. A mine has no count, and a count only exists where there's no mine:
/// the type itself rules out the meaningless "a mine with 3 neighbouring mines".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellContent {
    Mine,
    Empty { adjacent_mines: u8 },
}

impl CellContent {
    pub fn is_mine(self) -> bool {
        matches!(self, CellContent::Mine)
    }
}

/// `#[default]` only works on variants without data, so this one is written by hand.
impl Default for CellContent {
    fn default() -> Self {
        CellContent::Empty { adjacent_mines: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty_with_no_neighbouring_mines() {
        assert_eq!(CellContent::default(), CellContent::Empty { adjacent_mines: 0 });
    }

    #[test]
    fn only_a_mine_is_a_mine() {
        assert!(CellContent::Mine.is_mine());
        assert!(!CellContent::Empty { adjacent_mines: 0 }.is_mine());
        assert!(!CellContent::Empty { adjacent_mines: 8 }.is_mine());
    }
}
