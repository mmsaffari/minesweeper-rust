/// What a mouse gesture asks the game to do, before we know which cell it's on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    /// Left click: `Game::reveal`.
    Reveal,
    /// Right click: `Game::toggle_mark`.
    ToggleMark,
    /// Both buttons, or the middle one: `Game::chord`.
    Chord,
}
