use macroquad::math::{Rect, Vec2};

use super::{Command, theme};
use crate::logic::Difficulty;

/// One line in the dropdown.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenuItem {
    pub label: &'static str,
    /// Shown on the right, like "F2". Same keys as `keyboard.rs`.
    pub shortcut: &'static str,
    pub command: Command,
    pub rect: Rect,
}

/// The "Game" menu: the bar across the top of the window, its title, and the dropdown below.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuLayout {
    pub bar: Rect,
    pub title: Rect,
    pub dropdown: Rect,
    pub items: Vec<MenuItem>,
}

const ENTRIES: [(&str, &str, Command); 4] = [
    ("New", "F2", Command::NewGame),
    ("Beginner", "1", Command::Start(Difficulty::Beginner)),
    ("Intermediate", "2", Command::Start(Difficulty::Intermediate)),
    ("Expert", "3", Command::Start(Difficulty::Expert)),
];

impl MenuLayout {
    pub fn new(window_width: f32) -> Self {
        let bar_h = theme::MENU_BAR_HEIGHT;
        let item_h = theme::MENU_ITEM_HEIGHT;

        let items: Vec<MenuItem> = ENTRIES
            .iter()
            .enumerate()
            .map(|(i, &(label, shortcut, command))| MenuItem {
                label,
                shortcut,
                command,
                rect: Rect::new(0.0, bar_h + i as f32 * item_h, theme::MENU_WIDTH, item_h),
            })
            .collect();

        Self {
            bar: Rect::new(0.0, 0.0, window_width, bar_h),
            title: Rect::new(0.0, 0.0, theme::MENU_TITLE_WIDTH, bar_h),
            dropdown: Rect::new(0.0, bar_h, theme::MENU_WIDTH, items.len() as f32 * item_h),
            items,
        }
    }

    /// The dropdown line under `point`, if any.
    pub fn item_at(&self, point: Vec2) -> Option<&MenuItem> {
        self.items.iter().find(|item| item.rect.contains(point))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{BoardLayout, window_size};

    fn beginner_window() -> Vec2 {
        window_size(BoardLayout::for_difficulty(Difficulty::Beginner).rect())
    }

    fn menu() -> MenuLayout {
        MenuLayout::new(beginner_window().x)
    }

    #[test]
    fn bar_spans_the_window_top_with_the_title_on_the_left() {
        let menu = menu();
        assert_eq!(
            menu.bar,
            Rect::new(0.0, 0.0, beginner_window().x, theme::MENU_BAR_HEIGHT)
        );
        assert_eq!(menu.title.point(), Vec2::ZERO);
        assert_eq!(menu.title.h, menu.bar.h);
    }

    #[test]
    fn items_are_stacked_directly_below_the_bar_in_order() {
        let menu = menu();
        let commands: Vec<Command> = menu.items.iter().map(|item| item.command).collect();
        assert_eq!(
            commands,
            [
                Command::NewGame,
                Command::Start(Difficulty::Beginner),
                Command::Start(Difficulty::Intermediate),
                Command::Start(Difficulty::Expert),
            ]
        );
        let mut expected_top = menu.bar.bottom();
        for item in &menu.items {
            assert_eq!(item.rect.top(), expected_top, "{} leaves a gap or overlaps", item.label);
            expected_top = item.rect.bottom();
        }
        assert_eq!(
            menu.dropdown.bottom(),
            expected_top,
            "the dropdown wraps exactly its items"
        );
    }

    #[test]
    fn item_at_finds_the_item_under_the_point() {
        let menu = menu();
        for item in &menu.items {
            assert_eq!(menu.item_at(item.rect.center()), Some(item), "{}", item.label);
        }
        assert_eq!(menu.item_at(menu.title.center()), None, "the title isn't an item");
        let below = Vec2::new(10.0, menu.dropdown.bottom() + 5.0);
        assert_eq!(menu.item_at(below), None, "below the dropdown");
    }

    #[test]
    fn the_dropdown_fits_even_in_the_smallest_window() {
        let menu = menu();
        let window = beginner_window();
        assert!(menu.dropdown.right() <= window.x, "too wide for Beginner");
        assert!(menu.dropdown.bottom() <= window.y, "too tall for Beginner");
    }
}
