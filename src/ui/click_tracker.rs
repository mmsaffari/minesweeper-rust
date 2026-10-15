use macroquad::input::MouseButton;

use super::MouseAction;

/// Turns raw button presses/releases into game actions, the classic Windows way:
///
/// - **Left**: acts on *release* (so you can press, change your mind, and drag away).
/// - **Right**: acts on *press* (flags feel instant).
/// - **Both left and right held**: releasing either one chords. The *other* button's
///   release afterwards must do nothing; otherwise lifting the left button last would
///   also reveal.
/// - **Middle**: chords on release.
///
/// It needs memory between frames (which buttons are down, whether a chord already
/// happened), so it's a struct, not a function. It never touches the game or the
/// screen, so it can be tested with plain button events.
#[derive(Debug, Default)]
pub struct ClickTracker {
    left_down: bool,
    right_down: bool,
    /// A chord happened during this two-button gesture; the remaining release must be ignored.
    chorded: bool,
}

impl ClickTracker {
    pub fn press(&mut self, button: MouseButton) -> Option<MouseAction> {
        match button {
            MouseButton::Left => {
                self.left_down = true;
                None // a left click acts on release
            }
            MouseButton::Right => {
                self.right_down = true;
                if self.left_down {
                    None // both buttons down now: a chord is coming on release
                } else {
                    Some(MouseAction::ToggleMark)
                }
            }
            MouseButton::Middle | MouseButton::Unknown => None,
        }
    }

    pub fn release(&mut self, button: MouseButton) -> Option<MouseAction> {
        match button {
            MouseButton::Left | MouseButton::Right => {
                // Was this a two-button gesture? Check *before* forgetting this button.
                let both_were_down = self.left_down && self.right_down;
                match button {
                    MouseButton::Left => self.left_down = false,
                    _ => self.right_down = false,
                }

                let action = if both_were_down {
                    self.chorded = true;
                    Some(MouseAction::Chord)
                } else if self.chorded || button == MouseButton::Right {
                    None // the second release of a chord, or a right release (it acted on press)
                } else {
                    Some(MouseAction::Reveal)
                };

                // Both buttons up: the gesture is over, the next click starts fresh.
                if !self.left_down && !self.right_down {
                    self.chorded = false;
                }
                action
            }
            MouseButton::Middle => Some(MouseAction::Chord),
            MouseButton::Unknown => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MouseAction::{Chord, Reveal, ToggleMark};
    use MouseButton::{Left, Middle, Right};

    // --- helpers ---

    #[derive(Clone, Copy)]
    enum Event {
        Press(MouseButton),
        Release(MouseButton),
    }
    use Event::{Press, Release};

    /// Feeds a sequence of events to a fresh tracker and returns what each one produced.
    fn run(events: &[Event]) -> Vec<Option<MouseAction>> {
        let mut tracker = ClickTracker::default();
        events
            .iter()
            .map(|&event| match event {
                Press(button) => tracker.press(button),
                Release(button) => tracker.release(button),
            })
            .collect()
    }

    // --- single buttons ---

    #[test]
    fn left_click_reveals_on_release() {
        assert_eq!(run(&[Press(Left), Release(Left)]), [None, Some(Reveal)]);
    }

    #[test]
    fn right_click_marks_on_press() {
        assert_eq!(run(&[Press(Right), Release(Right)]), [Some(ToggleMark), None]);
    }

    #[test]
    fn middle_click_chords_on_release() {
        assert_eq!(run(&[Press(Middle), Release(Middle)]), [None, Some(Chord)]);
    }

    // --- both buttons ---

    #[test]
    fn left_then_right_chords_once_when_right_is_released_first() {
        assert_eq!(
            run(&[Press(Left), Press(Right), Release(Right), Release(Left)]),
            [None, None, Some(Chord), None],
            "the left release afterwards must not also reveal"
        );
    }

    #[test]
    fn left_then_right_chords_once_when_left_is_released_first() {
        assert_eq!(
            run(&[Press(Left), Press(Right), Release(Left), Release(Right)]),
            [None, None, Some(Chord), None]
        );
    }

    #[test]
    fn right_then_left_marks_on_press_and_chords_on_release() {
        // Like the original: the right press already toggles the mark. On an opened number
        // (where you'd chord) that does nothing, so the two don't get in each other's way.
        assert_eq!(
            run(&[Press(Right), Press(Left), Release(Left), Release(Right)]),
            [Some(ToggleMark), None, Some(Chord), None]
        );
    }

    #[test]
    fn a_normal_click_works_again_after_a_chord() {
        let events = [
            Press(Left),
            Press(Right),
            Release(Right),
            Release(Left), // chord done
            Press(Left),
            Release(Left), // plain left click
        ];
        assert_eq!(run(&events)[5], Some(Reveal));
    }

    #[test]
    fn unknown_buttons_do_nothing() {
        assert_eq!(
            run(&[Press(MouseButton::Unknown), Release(MouseButton::Unknown)]),
            [None, None]
        );
    }
}
