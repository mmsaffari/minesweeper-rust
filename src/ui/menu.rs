use macroquad::math::Vec2;

use super::{MenuClick, MenuLayout};

/// Whether the "Game" menu is open, and how clicks change that.
/// Like `ClickTracker`, it's pure state: no drawing, no real mouse, so it's testable.
#[derive(Debug, Default)]
pub struct Menu {
    open: bool,
}

impl Menu {
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    /// A left-button press at `point`.
    pub fn click(&mut self, point: Vec2, layout: &MenuLayout) -> MenuClick {
        if self.open {
            // Any click closes an open menu, and is swallowed, like in Windows:
            // clicking "away" from a menu must not also open a cell.
            self.open = false;
            match layout.item_at(point) {
                Some(item) => MenuClick::Chose(item.command),
                None => MenuClick::Consumed,
            }
        } else if layout.title.contains(point) {
            self.open = true;
            MenuClick::Consumed
        } else if layout.bar.contains(point) {
            MenuClick::Consumed // the empty part of the bar
        } else {
            MenuClick::NotMine
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::Difficulty;
    use crate::ui::Command;

    fn layout() -> MenuLayout {
        MenuLayout::new(300.0)
    }

    fn open_menu() -> Menu {
        Menu { open: true }
    }

    /// Below the menu bar and to the right of the dropdown: where the board would be.
    fn somewhere_on_the_board() -> Vec2 {
        Vec2::new(layout().dropdown.right() + 20.0, 200.0)
    }

    #[test]
    fn clicking_the_title_opens_and_closes_the_menu() {
        let layout = layout();
        let mut menu = Menu::default();
        assert_eq!(menu.click(layout.title.center(), &layout), MenuClick::Consumed);
        assert!(menu.is_open());
        assert_eq!(menu.click(layout.title.center(), &layout), MenuClick::Consumed);
        assert!(!menu.is_open());
    }

    #[test]
    fn choosing_an_item_returns_its_command_and_closes() {
        let layout = layout();
        let expert = layout.items[3];
        let mut menu = open_menu();
        assert_eq!(
            menu.click(expert.rect.center(), &layout),
            MenuClick::Chose(Command::Start(Difficulty::Expert))
        );
        assert!(!menu.is_open());
    }

    #[test]
    fn clicking_away_closes_the_menu_and_swallows_the_click() {
        let mut menu = open_menu();
        assert_eq!(menu.click(somewhere_on_the_board(), &layout()), MenuClick::Consumed);
        assert!(!menu.is_open());
    }

    #[test]
    fn a_closed_menu_ignores_clicks_on_the_board() {
        let mut menu = Menu::default();
        assert_eq!(menu.click(somewhere_on_the_board(), &layout()), MenuClick::NotMine);
        assert!(!menu.is_open());
    }

    #[test]
    fn the_empty_part_of_the_bar_does_nothing_but_still_owns_the_click() {
        let layout = layout();
        let mut menu = Menu::default();
        let empty_bar = Vec2::new(layout.bar.right() - 5.0, layout.bar.center().y);
        assert_eq!(menu.click(empty_bar, &layout), MenuClick::Consumed);
        assert!(!menu.is_open());
    }

    #[test]
    fn items_cannot_be_chosen_while_the_menu_is_closed() {
        let layout = layout();
        let mut menu = Menu::default();
        // The dropdown isn't visible, so its area is just the board/header behind it.
        assert_eq!(menu.click(layout.items[3].rect.center(), &layout), MenuClick::NotMine);
    }
}
