#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CellState {
    #[default]
    Hidden,
    Revealed,
    Flagged,
    Questioned,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_hidden() {
        let cell = CellState::default();
        assert_eq!(cell, CellState::Hidden);
    }
}
