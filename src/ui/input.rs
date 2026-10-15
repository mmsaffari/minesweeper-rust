use macroquad::input::{MouseButton, is_mouse_button_pressed, is_mouse_button_released, mouse_position};
use macroquad::math::Vec2;

use super::{BoardLayout, ClickTracker, MouseAction};
use crate::logic::Game;

/// Reads this frame's mouse button events and applies them to the game.
/// Call it once per frame, before drawing, so the frame already shows the result.
pub fn handle_mouse(game: &mut Game, layout: &BoardLayout, tracker: &mut ClickTracker) {
    for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
        if is_mouse_button_pressed(button) {
            apply(tracker.press(button), game, layout);
        }
        if is_mouse_button_released(button) {
            apply(tracker.release(button), game, layout);
        }
    }
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
