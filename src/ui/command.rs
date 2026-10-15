use crate::logic::Difficulty;

/// Something the player asks for outside the board: from a keyboard shortcut,
/// the face button, or (later) the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Start over with the same difficulty and a new random game number.
    NewGame,
    /// Play the current game number again: the same mines.
    Replay,
    /// Open the dialog to type a game number.
    ChooseGame,
    /// Play this game number (chosen in the dialog).
    StartNumbered(u32),
    /// Start over with this difficulty (the window resizes to fit).
    Start(Difficulty),
    /// Show the About dialog.
    About,
    /// Copy the board, as the player sees it, to the clipboard as text.
    CopyBoard,
    /// Point out a cell that is certainly safe, or certainly a mine.
    Hint,
}
