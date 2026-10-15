use macroquad::input::{
    MouseButton, get_char_pressed, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
    mouse_position,
};
use macroquad::math::Vec2;

use super::{BoardLayout, ClickTracker, HeaderLayout, MouseAction};
use crate::logic::Game;

const BUTTONS: [MouseButton; 3] = [MouseButton::Left, MouseButton::Right, MouseButton::Middle];

/// Reads this frame's mouse button events and, if `board_enabled`, applies them to the game.
/// The tracker always sees every event, even while the board is disabled (menu open);
/// otherwise it would think a button is still held down.
/// Call it once per frame, before drawing, so the frame already shows the result.
pub fn handle_mouse(game: &mut Game, layout: &BoardLayout, tracker: &mut ClickTracker, board_enabled: bool) {
    for button in BUTTONS {
        if is_mouse_button_pressed(button) {
            let action = tracker.press(button);
            if board_enabled {
                apply(action, game, layout);
            }
        }
        if is_mouse_button_released(button) {
            let action = tracker.release(button);
            if board_enabled {
                apply(action, game, layout);
            }
        }
    }
}

/// Any mouse button went down this frame.
pub fn any_button_pressed() -> bool {
    BUTTONS.into_iter().any(is_mouse_button_pressed)
}

/// Any mouse button is held down right now.
pub fn any_button_down() -> bool {
    BUTTONS.into_iter().any(is_mouse_button_down)
}

/// The left button was pressed this frame (where the menu looks for clicks).
pub fn left_pressed() -> bool {
    is_mouse_button_pressed(MouseButton::Left)
}

/// The left button was released this frame (where dialog buttons look for clicks).
pub fn left_released() -> bool {
    is_mouse_button_released(MouseButton::Left)
}

/// The characters typed since the last call. macroquad keeps them in a queue that only
/// empties when read, so the app reads it every frame: otherwise everything typed while
/// no text field is open would pour into the next one.
pub fn typed_chars() -> Vec<char> {
    std::iter::from_fn(get_char_pressed).collect()
}

pub fn mouse() -> Vec2 {
    Vec2::from(mouse_position())
}

/// The face button was clicked (released over it) this frame: time for a new game.
pub fn face_clicked(header: &HeaderLayout) -> bool {
    is_mouse_button_released(MouseButton::Left) && header.face.contains(Vec2::from(mouse_position()))
}

/// Turns "what" (the action) into "where" (the cell under the mouse) and tells the game.
/// Clicks outside the board, and button events that aren't actions, are ignored.
fn apply(action: Option<MouseAction>, game: &mut Game, layout: &BoardLayout) {
    let Some(action) = action else {
        return;
    };
    let Some(pos) = layout.position_at(Vec2::from(mouse_position())) else {
        return;
    };
    match action {
        MouseAction::Reveal => game.reveal(pos),
        MouseAction::ToggleMark => game.toggle_mark(pos),
        MouseAction::Chord => game.chord(pos),
    }
}
