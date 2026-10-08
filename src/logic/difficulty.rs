use super::BoardProperties;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Difficulty {
    #[default]
    Beginner,
    Intermediate,
    Expert,
}

impl Difficulty {
    pub fn board_properties(self) -> BoardProperties {
        match self {
            Difficulty::Beginner => BoardProperties {
                rows: 9,
                cols: 9,
                mines: 10,
            },
            Difficulty::Intermediate => BoardProperties {
                rows: 16,
                cols: 16,
                mines: 40,
            },
            Difficulty::Expert => BoardProperties {
                rows: 16,
                cols: 30,
                mines: 99,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- helpers ---

    const ALL: [Difficulty; 3] = [Difficulty::Beginner, Difficulty::Intermediate, Difficulty::Expert];

    fn props(rows: usize, cols: usize, mines: usize) -> BoardProperties {
        BoardProperties { rows, cols, mines }
    }

    // --- board_properties ---

    #[test]
    fn default_is_beginner() {
        assert_eq!(Difficulty::default(), Difficulty::Beginner);
    }

    #[test]
    fn board_properties_match_classic_minesweeper() {
        let cases = [
            (Difficulty::Beginner, props(9, 9, 10)),
            (Difficulty::Intermediate, props(16, 16, 40)),
            (Difficulty::Expert, props(16, 30, 99)),
        ];
        for (difficulty, expected) in cases {
            assert_eq!(difficulty.board_properties(), expected, "{difficulty:?}");
        }
    }

    #[test]
    fn every_difficulty_has_fewer_mines_than_cells() {
        for difficulty in ALL {
            let BoardProperties { rows, cols, mines } = difficulty.board_properties();
            assert!(mines < rows * cols, "{difficulty:?}: {mines} mines on {rows}x{cols}");
        }
    }
}
