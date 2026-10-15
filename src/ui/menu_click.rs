use super::Command;

/// What a left-button press did to the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuClick {
    /// The click wasn't on the menu, and the menu was closed: it belongs to the board/header.
    NotMine,
    /// The menu used the click (opened, closed, or nothing to do). It must not reach the board.
    Consumed,
    /// An item was chosen. The menu used the click, and here is the command.
    Chose(Command),
}
