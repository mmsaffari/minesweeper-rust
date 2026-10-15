use macroquad::math::Vec2;

use super::AboutLayout;

/// Whether the About dialog is open, and its OK button, which works like a real button:
/// it only fires if the press *and* the release are both on it. Pressing it, then dragging
/// away and letting go, cancels the click (like in Windows).
#[derive(Debug, Default)]
pub struct AboutDialog {
    open: bool,
    /// The left button went down on OK and hasn't been released yet.
    ok_pressed: bool,
}

impl AboutDialog {
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn show(&mut self) {
        self.open = true;
        self.ok_pressed = false;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.ok_pressed = false;
    }

    /// For drawing: OK is held down.
    pub fn ok_pressed(&self) -> bool {
        self.ok_pressed
    }

    /// A left-button press at `point`. While the dialog is open, every click belongs to it.
    pub fn press(&mut self, point: Vec2, layout: &AboutLayout) {
        self.ok_pressed = layout.ok_button.contains(point);
    }

    /// A left-button release at `point`: closes the dialog if this completes a click on OK.
    pub fn release(&mut self, point: Vec2, layout: &AboutLayout) {
        if self.ok_pressed && layout.ok_button.contains(point) {
            self.open = false;
        }
        self.ok_pressed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout() -> AboutLayout {
        AboutLayout::centered_in(Vec2::new(400.0, 400.0))
    }

    fn shown() -> AboutDialog {
        let mut dialog = AboutDialog::default();
        dialog.show();
        dialog
    }

    fn ok() -> Vec2 {
        layout().ok_button.center()
    }

    fn elsewhere() -> Vec2 {
        layout().text.center()
    }

    #[test]
    fn clicking_ok_closes_it() {
        let mut dialog = shown();
        dialog.press(ok(), &layout());
        assert!(dialog.ok_pressed(), "OK looks pushed in while held");
        assert!(dialog.is_open(), "only the release closes it");
        dialog.release(ok(), &layout());
        assert!(!dialog.is_open());
    }

    #[test]
    fn dragging_off_ok_before_letting_go_cancels() {
        let mut dialog = shown();
        dialog.press(ok(), &layout());
        dialog.release(elsewhere(), &layout());
        assert!(dialog.is_open());
        assert!(!dialog.ok_pressed());
    }

    #[test]
    fn releasing_on_ok_after_pressing_elsewhere_does_nothing() {
        let mut dialog = shown();
        dialog.press(elsewhere(), &layout());
        assert!(!dialog.ok_pressed());
        dialog.release(ok(), &layout());
        assert!(dialog.is_open());
    }
}
