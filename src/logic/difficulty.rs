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

    #[test]
    fn beginner_geometry_is_correct() {
        let difficulty = Difficulty::Beginner;
        let properties = difficulty.board_properties();
        assert_eq!(properties.rows, 9);
        assert_eq!(properties.cols, 9);
        assert_eq!(properties.mines, 10);
    }
    #[test]
    fn intermediate_geometry_is_correct() {
        let difficulty = Difficulty::Intermediate;
        let properties = difficulty.board_properties();
        assert_eq!(properties.rows, 16);
        assert_eq!(properties.cols, 16);
        assert_eq!(properties.mines, 40);
    }
    #[test]
    fn expert_geometry_is_correct() {
        let difficulty = Difficulty::Expert;
        let properties = difficulty.board_properties();
        assert_eq!(properties.rows, 16);
        assert_eq!(properties.cols, 30);
        assert_eq!(properties.mines, 99);
    }

    #[test]
    fn there_are_less_mines_than_cells() {
        for difficulty in [
            Difficulty::Beginner,
            Difficulty::Intermediate,
            Difficulty::Expert,
        ] {
            let properties = difficulty.board_properties();
            assert!(properties.rows * properties.cols > properties.mines);
        }
    }
}
