//! Everything that works from what the player can see, never from where the mines are:
//! the board export (`View`'s text form) and the hint.

mod clue;
mod find_hint;
mod hint;
mod seen;
mod view;

pub use clue::Clue;
pub use find_hint::find_hint;
pub use hint::Hint;
pub use seen::Seen;
pub use view::View;
