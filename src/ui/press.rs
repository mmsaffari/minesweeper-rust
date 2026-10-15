/// What the mouse buttons are holding down right now, for the "pushed in" look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Press {
    /// Nothing held (or only the right button, which acts on press and shows nothing).
    None,
    /// The left button: the cell under the mouse looks pushed in.
    Single,
    /// Both buttons, or the middle one: the cell under the mouse and its neighbours do.
    Chord,
}
