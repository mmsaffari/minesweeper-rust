use macroquad::math::{Rect, Vec2};

use super::theme;

/// Where the parts of the About dialog go: a panel centred in the window, as wide as fits
/// (up to a maximum), with a title bar, the mine picture, the text and an OK button.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AboutLayout {
    pub panel: Rect,
    pub title_bar: Rect,
    pub icon: Rect,
    pub text: Rect,
    pub ok_button: Rect,
}

impl AboutLayout {
    pub fn centered_in(window: Vec2) -> Self {
        let w = (window.x - 2.0 * theme::SIDE_MARGIN).min(theme::ABOUT_MAX_WIDTH);
        let h = theme::ABOUT_HEIGHT;
        let panel = Rect::new((window.x - w) / 2.0, (window.y - h) / 2.0, w, h);

        let b = theme::BUTTON_BEVEL;
        let pad = theme::ABOUT_PADDING;
        let title_bar = Rect::new(panel.x + b, panel.y + b, w - 2.0 * b, theme::ABOUT_TITLE_HEIGHT);
        let icon = Rect::new(panel.x, title_bar.bottom() + pad, w, theme::ABOUT_ICON_HEIGHT);

        let (button_w, button_h) = (theme::ABOUT_BUTTON_WIDTH, theme::ABOUT_BUTTON_HEIGHT);
        let ok_button = Rect::new(
            panel.center().x - button_w / 2.0,
            panel.bottom() - pad - button_h,
            button_w,
            button_h,
        );

        let text_top = icon.bottom() + pad;
        let text = Rect::new(panel.x + pad, text_top, w - 2.0 * pad, ok_button.top() - pad - text_top);

        Self {
            panel,
            title_bar,
            icon,
            text,
            ok_button,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::Difficulty;
    use crate::ui::{BoardLayout, window_size};

    fn window(difficulty: Difficulty) -> Vec2 {
        window_size(BoardLayout::for_difficulty(difficulty).rect())
    }

    fn inside(outer: Rect, inner: Rect) -> bool {
        inner.left() >= outer.left()
            && inner.right() <= outer.right()
            && inner.top() >= outer.top()
            && inner.bottom() <= outer.bottom()
    }

    #[test]
    fn fits_and_is_centred_in_every_window() {
        for difficulty in [Difficulty::Beginner, Difficulty::Intermediate, Difficulty::Expert] {
            let window = window(difficulty);
            let about = AboutLayout::centered_in(window);
            let screen = Rect::new(0.0, 0.0, window.x, window.y);
            assert!(
                inside(screen, about.panel),
                "{difficulty:?}: {:?} sticks out",
                about.panel
            );
            assert_eq!(about.panel.center(), screen.center(), "{difficulty:?}");
        }
    }

    #[test]
    fn is_never_wider_than_the_maximum() {
        let about = AboutLayout::centered_in(window(Difficulty::Expert));
        assert_eq!(about.panel.w, theme::ABOUT_MAX_WIDTH);
    }

    #[test]
    fn parts_are_stacked_top_to_bottom_inside_the_panel() {
        let about = AboutLayout::centered_in(window(Difficulty::Beginner));
        for part in [about.title_bar, about.icon, about.text, about.ok_button] {
            assert!(inside(about.panel, part), "{part:?} sticks out of the panel");
        }
        assert!(about.title_bar.bottom() <= about.icon.top());
        assert!(about.icon.bottom() <= about.text.top());
        assert!(about.text.bottom() <= about.ok_button.top());
        assert!(about.text.h > 0.0, "no room left for the text");
        assert_eq!(about.ok_button.center().x, about.panel.center().x, "OK is centred");
    }
}
