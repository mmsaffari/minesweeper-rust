use macroquad::math::{Rect, Vec2};

use super::MenuItem;

/// One menu: its title in the bar ("Game", "Help") and the dropdown that opens below it.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuColumn {
    pub title: &'static str,
    pub title_rect: Rect,
    pub dropdown: Rect,
    pub items: Vec<MenuItem>,
}

impl MenuColumn {
    /// The dropdown line under `point`, if any.
    pub fn item_at(&self, point: Vec2) -> Option<&MenuItem> {
        self.items.iter().find(|item| item.rect.contains(point))
    }
}
