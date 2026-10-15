//! Everything that works from what the player can see, never from where the mines are:
//! the board export (`View`'s text form) and the hint.

mod seen;
mod view;

pub use seen::Seen;
pub use view::View;
