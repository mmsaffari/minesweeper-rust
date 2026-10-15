use macroquad::math::{Rect, Vec2};

use super::{Command, MenuColumn, MenuItem, theme};
use crate::logic::Difficulty;

/// The menu bar across the top of the window, and every menu in it, left to right.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuLayout {
    pub bar: Rect,
    pub menus: Vec<MenuColumn>,
}

type Entry = (&'static str, &'static str, Command);

const GAME: [Entry; 8] = [
    ("New", "F2", Command::NewGame),
    ("Replay", "R", Command::Replay),
    ("Choose game...", "G", Command::ChooseGame),
    ("Beginner", "1", Command::Start(Difficulty::Beginner)),
    ("Intermediate", "2", Command::Start(Difficulty::Intermediate)),
    ("Expert", "3", Command::Start(Difficulty::Expert)),
    ("Hint", "H", Command::Hint),
    ("Copy board", "Ctrl+C", Command::CopyBoard),
];

const HELP: [Entry; 1] = [("About Minesweeper", "F1", Command::About)];

const MENUS: [(&str, &[Entry]); 2] = [("Game", &GAME), ("Help", &HELP)];

impl MenuLayout {
    pub fn new(window_width: f32) -> Self {
        let bar_h = theme::MENU_BAR_HEIGHT;
        let item_h = theme::MENU_ITEM_HEIGHT;

        let menus = MENUS
            .iter()
            .enumerate()
            .map(|(i, &(title, entries))| {
                let title_rect = Rect::new(i as f32 * theme::MENU_TITLE_WIDTH, 0.0, theme::MENU_TITLE_WIDTH, bar_h);
                // Open below its title, but pushed left if it would stick out of the window.
                let x = title_rect.x.min(window_width - theme::MENU_WIDTH).max(0.0);
                let items = entries
                    .iter()
                    .enumerate()
                    .map(|(row, &(label, shortcut, command))| MenuItem {
                        label,
                        shortcut,
                        command,
                        rect: Rect::new(x, bar_h + row as f32 * item_h, theme::MENU_WIDTH, item_h),
                    })
                    .collect::<Vec<_>>();
                MenuColumn {
                    title,
                    title_rect,
                    dropdown: Rect::new(x, bar_h, theme::MENU_WIDTH, items.len() as f32 * item_h),
                    items,
                }
            })
            .collect();

        Self {
            bar: Rect::new(0.0, 0.0, window_width, bar_h),
            menus,
        }
    }

    /// Which menu's title is under `point`, by index into `menus`.
    pub fn title_at(&self, point: Vec2) -> Option<usize> {
        self.menus.iter().position(|menu| menu.title_rect.contains(point))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{BoardLayout, window_size};

    fn beginner_window() -> Vec2 {
        window_size(BoardLayout::for_difficulty(Difficulty::Beginner).rect())
    }

    fn layout() -> MenuLayout {
        MenuLayout::new(beginner_window().x)
    }

    fn commands(menu: &MenuColumn) -> Vec<Command> {
        menu.items.iter().map(|item| item.command).collect()
    }

    #[test]
    fn bar_spans_the_window_top() {
        assert_eq!(
            layout().bar,
            Rect::new(0.0, 0.0, beginner_window().x, theme::MENU_BAR_HEIGHT)
        );
    }

    #[test]
    fn titles_sit_side_by_side_in_the_bar_game_first() {
        let layout = layout();
        let titles: Vec<&str> = layout.menus.iter().map(|menu| menu.title).collect();
        assert_eq!(titles, ["Game", "Help"]);
        assert_eq!(layout.menus[0].title_rect.left(), 0.0);
        assert_eq!(layout.menus[1].title_rect.left(), layout.menus[0].title_rect.right());
        for menu in &layout.menus {
            assert_eq!(menu.title_rect.h, layout.bar.h, "{}", menu.title);
        }
    }

    #[test]
    fn each_menu_has_its_commands_in_order() {
        let layout = layout();
        assert_eq!(
            commands(&layout.menus[0]),
            [
                Command::NewGame,
                Command::Replay,
                Command::ChooseGame,
                Command::Start(Difficulty::Beginner),
                Command::Start(Difficulty::Intermediate),
                Command::Start(Difficulty::Expert),
                Command::Hint,
                Command::CopyBoard,
            ]
        );
        assert_eq!(commands(&layout.menus[1]), [Command::About]);
    }

    #[test]
    fn items_are_stacked_directly_below_the_bar() {
        for menu in &layout().menus {
            let mut expected_top = theme::MENU_BAR_HEIGHT;
            for item in &menu.items {
                assert_eq!(item.rect.top(), expected_top, "{} leaves a gap or overlaps", item.label);
                assert_eq!(
                    item.rect.x, menu.dropdown.x,
                    "{} sticks out of its dropdown",
                    item.label
                );
                expected_top = item.rect.bottom();
            }
            assert_eq!(
                menu.dropdown.bottom(),
                expected_top,
                "{}: the dropdown wraps its items",
                menu.title
            );
        }
    }

    #[test]
    fn title_at_and_item_at_find_what_is_under_the_point() {
        let layout = layout();
        for (index, menu) in layout.menus.iter().enumerate() {
            assert_eq!(layout.title_at(menu.title_rect.center()), Some(index), "{}", menu.title);
            for item in &menu.items {
                assert_eq!(menu.item_at(item.rect.center()), Some(item), "{}", item.label);
            }
        }
        let empty_bar = Vec2::new(layout.bar.right() - 2.0, layout.bar.center().y);
        assert_eq!(layout.title_at(empty_bar), None);
    }

    /// The Help menu's title is not at the left edge, so its dropdown could stick out
    /// of the narrow Beginner window; it must be pushed back inside.
    #[test]
    fn every_dropdown_fits_even_in_the_smallest_window() {
        let window = beginner_window();
        for menu in &layout().menus {
            assert!(menu.dropdown.left() >= 0.0, "{} starts left of the window", menu.title);
            assert!(
                menu.dropdown.right() <= window.x,
                "{} is too wide for Beginner",
                menu.title
            );
            assert!(
                menu.dropdown.bottom() <= window.y,
                "{} is too tall for Beginner",
                menu.title
            );
        }
    }
}
