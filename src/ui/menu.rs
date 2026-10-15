use macroquad::math::Vec2;

use super::{MenuClick, MenuLayout};

/// Which menu is open (if any), and how clicks change that.
/// Like `ClickTracker`, it's pure state: no drawing, no real mouse, so it's testable.
#[derive(Debug, Default)]
pub struct Menu {
    /// Index into `MenuLayout::menus`.
    open: Option<usize>,
}

impl Menu {
    pub fn is_open(&self) -> bool {
        self.open.is_some()
    }

    pub fn open_menu(&self) -> Option<usize> {
        self.open
    }

    pub fn close(&mut self) {
        self.open = None;
    }

    /// A left-button press at `point`.
    pub fn click(&mut self, point: Vec2, layout: &MenuLayout) -> MenuClick {
        let clicked_title = layout.title_at(point);

        if let Some(open) = self.open.take() {
            // Any click closes an open menu, and is swallowed, like in Windows:
            // clicking "away" from a menu must not also open a cell.
            if let Some(item) = layout.menus[open].item_at(point) {
                return MenuClick::Chose(item.command);
            }
            // Clicking another menu's title switches straight to that menu.
            if let Some(other) = clicked_title
                && other != open
            {
                self.open = Some(other);
            }
            MenuClick::Consumed
        } else if clicked_title.is_some() {
            self.open = clicked_title;
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

    const GAME: usize = 0;
    const HELP: usize = 1;

    fn layout() -> MenuLayout {
        MenuLayout::new(400.0)
    }

    fn opened(menu: usize) -> Menu {
        Menu { open: Some(menu) }
    }

    fn title(menu: usize) -> Vec2 {
        layout().menus[menu].title_rect.center()
    }

    /// Below the menu bar and to the right of every dropdown: where the board would be.
    fn somewhere_on_the_board() -> Vec2 {
        let right_of_menus = layout().menus.iter().map(|m| m.dropdown.right()).fold(0.0, f32::max);
        Vec2::new(right_of_menus + 20.0, 200.0)
    }

    #[test]
    fn clicking_a_title_opens_its_menu_and_clicking_it_again_closes_it() {
        let layout = layout();
        let mut menu = Menu::default();
        assert_eq!(menu.click(title(HELP), &layout), MenuClick::Consumed);
        assert_eq!(menu.open_menu(), Some(HELP));
        assert_eq!(menu.click(title(HELP), &layout), MenuClick::Consumed);
        assert!(!menu.is_open());
    }

    #[test]
    fn clicking_another_title_switches_menus() {
        let mut menu = opened(GAME);
        assert_eq!(menu.click(title(HELP), &layout()), MenuClick::Consumed);
        assert_eq!(menu.open_menu(), Some(HELP));
    }

    #[test]
    fn choosing_an_item_returns_its_command_and_closes() {
        let layout = layout();

        let mut menu = opened(GAME);
        let expert = layout.menus[GAME].items[3];
        assert_eq!(
            menu.click(expert.rect.center(), &layout),
            MenuClick::Chose(Command::Start(Difficulty::Expert))
        );
        assert!(!menu.is_open());

        let mut menu = opened(HELP);
        let about = layout.menus[HELP].items[0];
        assert_eq!(
            menu.click(about.rect.center(), &layout),
            MenuClick::Chose(Command::About)
        );
    }

    #[test]
    fn only_the_open_menus_items_can_be_chosen() {
        let layout = layout();
        let mut menu = opened(GAME);
        let about = layout.menus[HELP].items[0].rect.center();
        // Wherever "About" would be, the Game menu is open; at most a Game item is hit.
        assert_ne!(menu.click(about, &layout), MenuClick::Chose(Command::About));
    }

    #[test]
    fn clicking_away_closes_the_menu_and_swallows_the_click() {
        let mut menu = opened(GAME);
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
        let expert = layout.menus[GAME].items[3].rect.center();
        assert_eq!(menu.click(expert, &layout), MenuClick::NotMine);
    }
}
