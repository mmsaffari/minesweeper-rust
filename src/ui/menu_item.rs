use macroquad::math::Rect;

use super::Command;

/// One line in a dropdown.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenuItem {
    pub label: &'static str,
    /// Shown on the right, like "F2". Same keys as `keyboard.rs`.
    pub shortcut: &'static str,
    pub command: Command,
    pub rect: Rect,
}
