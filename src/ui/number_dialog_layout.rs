use macroquad::math::{Rect, Vec2};

use super::{DialogButton, theme};

/// Where the parts of the "Choose game" dialog go: a panel centred in the window with a
/// title bar, a label, the number field, and OK / Cancel side by side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberDialogLayout {
    pub panel: Rect,
    pub title_bar: Rect,
    pub label: Rect,
    pub field: Rect,
    pub ok: Rect,
    pub cancel: Rect,
}

impl NumberDialogLayout {
    pub fn centered_in(window: Vec2) -> Self {
        let pad = theme::ABOUT_PADDING;
        let b = theme::BUTTON_BEVEL;
        let (button_w, button_h) = (theme::ABOUT_BUTTON_WIDTH, theme::ABOUT_BUTTON_HEIGHT);

        let w = (window.x - 2.0 * theme::SIDE_MARGIN).min(theme::NUMBER_DIALOG_WIDTH);
        let h = b
            + theme::ABOUT_TITLE_HEIGHT
            + pad
            + theme::NUMBER_LABEL_HEIGHT
            + pad
            + theme::NUMBER_FIELD_HEIGHT
            + pad
            + button_h
            + pad
            + b;
        let panel = Rect::new((window.x - w) / 2.0, (window.y - h) / 2.0, w, h);

        let title_bar = Rect::new(panel.x + b, panel.y + b, w - 2.0 * b, theme::ABOUT_TITLE_HEIGHT);
        let inner_x = panel.x + pad;
        let inner_w = w - 2.0 * pad;
        let label = Rect::new(inner_x, title_bar.bottom() + pad, inner_w, theme::NUMBER_LABEL_HEIGHT);
        let field = Rect::new(inner_x, label.bottom() + pad, inner_w, theme::NUMBER_FIELD_HEIGHT);

        let buttons_y = field.bottom() + pad;
        let gap = pad;
        let left = panel.center().x - button_w - gap / 2.0;
        let ok = Rect::new(left, buttons_y, button_w, button_h);
        let cancel = Rect::new(left + button_w + gap, buttons_y, button_w, button_h);

        Self {
            panel,
            title_bar,
            label,
            field,
            ok,
            cancel,
        }
    }

    pub fn button(&self, button: DialogButton) -> Rect {
        match button {
            DialogButton::Ok => self.ok,
            DialogButton::Cancel => self.cancel,
        }
    }

    /// The button under `point`, if any.
    pub fn button_at(&self, point: Vec2) -> Option<DialogButton> {
        [DialogButton::Ok, DialogButton::Cancel]
            .into_iter()
            .find(|&button| self.button(button).contains(point))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::Difficulty;
    use crate::ui::{BoardLayout, window_size};

    fn layout(difficulty: Difficulty) -> NumberDialogLayout {
        NumberDialogLayout::centered_in(window_size(BoardLayout::for_difficulty(difficulty).rect()))
    }

    fn inside(outer: Rect, inner: Rect) -> bool {
        inner.left() >= outer.left()
            && inner.right() <= outer.right()
            && inner.top() >= outer.top()
            && inner.bottom() <= outer.bottom()
    }

    #[test]
    fn fits_in_every_window_with_everything_inside_the_panel() {
        for difficulty in [Difficulty::Beginner, Difficulty::Intermediate, Difficulty::Expert] {
            let window = window_size(BoardLayout::for_difficulty(difficulty).rect());
            let dialog = layout(difficulty);
            assert!(
                inside(Rect::new(0.0, 0.0, window.x, window.y), dialog.panel),
                "{difficulty:?}"
            );
            for part in [dialog.title_bar, dialog.label, dialog.field, dialog.ok, dialog.cancel] {
                assert!(inside(dialog.panel, part), "{difficulty:?}: {part:?} sticks out");
            }
        }
    }

    #[test]
    fn parts_are_stacked_and_the_buttons_sit_side_by_side() {
        let dialog = layout(Difficulty::Beginner);
        assert!(dialog.title_bar.bottom() <= dialog.label.top());
        assert!(dialog.label.bottom() <= dialog.field.top());
        assert!(dialog.field.bottom() <= dialog.ok.top());
        assert_eq!(dialog.ok.y, dialog.cancel.y);
        assert!(dialog.ok.right() < dialog.cancel.left(), "OK and Cancel overlap");
    }

    #[test]
    fn button_at_finds_each_button_and_nothing_elsewhere() {
        let dialog = layout(Difficulty::Beginner);
        assert_eq!(dialog.button_at(dialog.ok.center()), Some(DialogButton::Ok));
        assert_eq!(dialog.button_at(dialog.cancel.center()), Some(DialogButton::Cancel));
        assert_eq!(dialog.button_at(dialog.field.center()), None);
    }
}
