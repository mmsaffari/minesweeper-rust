use crate::logic::Position;

/// What the solver can say for certain about the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hint {
    /// This cell can't be a mine: it's safe to open.
    Safe(Position),
    /// This cell must be a mine: it can be flagged.
    Mine(Position),
    /// Nothing is certain from what's visible: the next move is a guess.
    NoSureMove,
}
