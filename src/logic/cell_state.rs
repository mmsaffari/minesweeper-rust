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
        assert_eq!(CellState::default(), CellState::Hidden);
    }
}
