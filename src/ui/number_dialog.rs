use macroquad::math::Vec2;

use super::{DialogButton, NumberDialogLayout};
use crate::logic::MAX_GAME_NUMBER;

/// The "Choose game" dialog: whether it's open, the number being typed, and its buttons.
/// Pure state, like `AboutDialog`: tests type into it and click it without a window.
#[derive(Debug, Default)]
pub struct NumberDialog {
    open: bool,
    digits: String,
    /// The field still shows the current game's number, untouched: the first digit typed
    /// replaces it instead of being added to it (like a selected text field).
    untouched: bool,
    /// The button the left mouse button went down on, if any.
    pressed: Option<DialogButton>,
}

impl NumberDialog {
    /// Opens it, showing `current` (the game being played) as the starting value.
    pub fn show(&mut self, current: u32) {
        *self = Self {
            open: true,
            digits: current.to_string(),
            untouched: true,
            pressed: None,
        };
    }

    pub fn close(&mut self) {
        self.open = false;
        self.pressed = None;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn digits(&self) -> &str {
        &self.digits
    }

    pub fn pressed(&self) -> Option<DialogButton> {
        self.pressed
    }

    /// The number typed, if it's a valid game number (1 to `MAX_GAME_NUMBER`).
    pub fn number(&self) -> Option<u32> {
        self.digits.parse().ok().filter(|n| (1..=MAX_GAME_NUMBER).contains(n))
    }

    /// A typed character. Only digits count, and only as many as the biggest number has.
    pub fn type_char(&mut self, c: char) {
        if !c.is_ascii_digit() {
            return;
        }
        if self.untouched {
            self.digits.clear();
            self.untouched = false;
        }
        if self.digits.len() < MAX_GAME_NUMBER.to_string().len() {
            self.digits.push(c);
        }
    }

    pub fn backspace(&mut self) {
        self.untouched = false;
        self.digits.pop();
    }

    /// A left-button press at `point`.
    pub fn press(&mut self, point: Vec2, layout: &NumberDialogLayout) {
        self.pressed = layout.button_at(point);
    }

    /// A left-button release at `point`: the button clicked, if the press and the release
    /// were both on it (drag off a button before letting go, and nothing happens).
    pub fn release(&mut self, point: Vec2, layout: &NumberDialogLayout) -> Option<DialogButton> {
        let clicked = self.pressed.filter(|&button| layout.button(button).contains(point));
        self.pressed = None;
        clicked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shown(current: u32) -> NumberDialog {
        let mut dialog = NumberDialog::default();
        dialog.show(current);
        dialog
    }

    fn type_text(dialog: &mut NumberDialog, text: &str) {
        text.chars().for_each(|c| dialog.type_char(c));
    }

    fn layout() -> NumberDialogLayout {
        NumberDialogLayout::centered_in(Vec2::new(400.0, 400.0))
    }

    #[test]
    fn opens_with_the_current_game_number() {
        let dialog = shown(4711);
        assert!(dialog.is_open());
        assert_eq!(dialog.digits(), "4711");
        assert_eq!(dialog.number(), Some(4711));
    }

    #[test]
    fn the_first_digit_typed_replaces_the_shown_number() {
        let mut dialog = shown(4711);
        type_text(&mut dialog, "42");
        assert_eq!(dialog.number(), Some(42));
    }

    #[test]
    fn backspace_edits_the_shown_number_instead() {
        let mut dialog = shown(4711);
        dialog.backspace();
        type_text(&mut dialog, "9");
        assert_eq!(dialog.digits(), "4719");
    }

    #[test]
    fn only_digits_are_typed_and_no_more_than_six() {
        let mut dialog = shown(1);
        type_text(&mut dialog, "12a-3 4567");
        assert_eq!(dialog.digits(), "123456");
    }

    #[test]
    fn only_numbers_from_1_to_the_maximum_are_valid() {
        let mut dialog = shown(1);
        for (typed, expected) in [
            ("0", None),
            ("000", None),
            ("1", Some(1)),
            ("999999", Some(MAX_GAME_NUMBER)),
        ] {
            dialog.show(1);
            type_text(&mut dialog, typed);
            assert_eq!(dialog.number(), expected, "{typed:?}");
        }
        dialog.show(1);
        dialog.backspace();
        assert_eq!(dialog.number(), None, "an empty field");
    }

    #[test]
    fn a_button_is_clicked_by_pressing_and_releasing_on_it() {
        let layout = layout();
        let mut dialog = shown(1);

        dialog.press(layout.ok.center(), &layout);
        assert_eq!(dialog.pressed(), Some(DialogButton::Ok), "looks pushed in while held");
        assert_eq!(dialog.release(layout.ok.center(), &layout), Some(DialogButton::Ok));

        dialog.press(layout.cancel.center(), &layout);
        assert_eq!(
            dialog.release(layout.field.center(), &layout),
            None,
            "dragged off: cancelled"
        );
        assert_eq!(dialog.pressed(), None);
    }
}
