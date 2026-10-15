use crate::logic::Difficulty;

/// Something the player asks for outside the board: from a keyboard shortcut,
/// the face button, or (later) the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Start over with the same difficulty.
    NewGame,
    /// Start over with this difficulty (the window resizes to fit).
    Start(Difficulty),
    /// Show the About dialog.
    About,
    /// Copy the board, as the player sees it, to the clipboard as text.
    CopyBoard,
}
