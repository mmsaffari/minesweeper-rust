use super::CellState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cell {
    is_mine: bool,
    adjacent_mines: u8,
    state: CellState,
}

impl Cell {
    pub fn cycle_mark(&mut self) {
        self.state = match self.state {
            CellState::Revealed => CellState::Revealed,
            CellState::Hidden => CellState::Flagged,
            CellState::Flagged => CellState::Questioned,
            CellState::Questioned => CellState::Hidden,
        };
    }

    pub fn reveal(&mut self) {
        self.state = match self.state {
            CellState::Flagged => CellState::Flagged,
            CellState::Hidden => CellState::Revealed,
            CellState::Questioned => CellState::Revealed,
            CellState::Revealed => CellState::Revealed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_mark_ignores_revealed() {
        let mut cell = Cell {
            state: CellState::Revealed,
            ..Default::default()
        };
        cell.cycle_mark();
        assert_eq!(cell.state, CellState::Revealed);
    }

    #[test]
    fn cycle_mark_changes_hidden_to_flagged() {
        let mut cell = Cell {
            state: CellState::Hidden,
            ..Default::default()
        };
        cell.cycle_mark();
        assert_eq!(cell.state, CellState::Flagged);
    }

    #[test]
    fn cycle_mark_changes_flagged_to_questioned() {
        let mut cell = Cell {
            state: CellState::Flagged,
            ..Default::default()
        };
        cell.cycle_mark();
        assert_eq!(cell.state, CellState::Questioned);
    }

    #[test]
    fn cycle_mark_changes_questioned_to_hidden() {
        let mut cell = Cell {
            state: CellState::Questioned,
            ..Default::default()
        };
        cell.cycle_mark();
        assert_eq!(cell.state, CellState::Hidden);
    }

    #[test]
    fn reveal_ignores_flagged_cell() {
        let mut cell = Cell {
            state: CellState::Flagged,
            ..Default::default()
        };
        cell.reveal();
        assert_eq!(cell.state, CellState::Flagged);
    }

    #[test]
    fn reveal_reveals_non_flagged_cell() {
        for state in [
            CellState::Hidden,
            CellState::Questioned,
            CellState::Revealed,
        ] {
            let mut cell = Cell {
                state,
                ..Default::default()
            };
            cell.reveal();
            assert_eq!(cell.state, CellState::Revealed);
        }
    }

    #[test]
    fn cell_default_state_is_hidden() {
        let cell = Cell::default();
        assert!(!cell.is_mine);
        assert_eq!(cell.adjacent_mines, 0);
        assert_eq!(cell.state, CellState::Hidden);
    }
}
