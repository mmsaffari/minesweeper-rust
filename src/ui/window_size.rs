use macroquad::math::{Rect, Vec2};

use super::theme;

/// How big the window must be to hold the board (`BoardLayout::rect()`) plus its margins
/// and the header above it.
pub fn window_size(board: Rect) -> Vec2 {
    Vec2::new(
        theme::SIDE_MARGIN + board.w + theme::SIDE_MARGIN,
        theme::TOP_MARGIN + board.h + theme::BOTTOM_MARGIN,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_the_margins_around_the_board() {
        let board = Rect::new(theme::SIDE_MARGIN, theme::TOP_MARGIN, 216.0, 216.0);
        let expected = Vec2::new(
            216.0 + 2.0 * theme::SIDE_MARGIN,
            216.0 + theme::TOP_MARGIN + theme::BOTTOM_MARGIN,
        );
        assert_eq!(window_size(board), expected);
    }
}
