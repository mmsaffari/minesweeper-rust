use super::CellState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cell {
    pub(super) is_mine: bool,
    pub(super) adjacent_mines: u8,
    pub(super) state: CellState,
}

impl Cell {
    /// Read-only, for the UI (which lives outside `logic`, so it can't see the fields).
    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }

    pub fn adjacent_mines(&self) -> u8 {
        self.adjacent_mines
    }

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
    use CellState::{Flagged, Hidden, Questioned, Revealed};

    // --- helpers ---

    fn cell_in(state: CellState) -> Cell {
        Cell {
            state,
            ..Default::default()
        }
    }

    // --- getters ---

    #[test]
    fn getters_return_the_fields() {
        let cell = Cell {
            is_mine: true,
            adjacent_mines: 3,
            state: Flagged,
        };
        assert_eq!(
            (cell.state(), cell.is_mine(), cell.adjacent_mines()),
            (Flagged, true, 3)
        );
    }

    // --- default ---

    #[test]
    fn default_cell_is_hidden_mine_free_and_zero_count() {
        let cell = Cell::default();
        assert!(!cell.is_mine);
        assert_eq!(cell.adjacent_mines, 0);
        assert_eq!(cell.state, Hidden);
    }

    // --- cycle_mark ---

    #[test]
    fn cycle_mark_goes_hidden_flagged_questioned_hidden_and_ignores_revealed() {
        for (from, to) in [
            (Hidden, Flagged),
            (Flagged, Questioned),
            (Questioned, Hidden),
            (Revealed, Revealed),
        ] {
            let mut cell = cell_in(from);
            cell.cycle_mark();
            assert_eq!(cell.state, to, "cycle_mark from {from:?}");
        }
    }

    // --- reveal ---

    #[test]
    fn reveal_opens_everything_except_flagged() {
        for (from, to) in [
            (Hidden, Revealed),
            (Questioned, Revealed),
            (Revealed, Revealed),
            (Flagged, Flagged),
        ] {
            let mut cell = cell_in(from);
            cell.reveal();
            assert_eq!(cell.state, to, "reveal from {from:?}");
        }
    }
}
