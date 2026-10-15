use macroquad::math::Rect;

use super::theme;

/// Where the mine counter, the face button and the timer sit:
/// a strip in the margin above the board, exactly as wide as the board.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeaderLayout {
    pub counter: Rect,
    pub face: Rect,
    pub timer: Rect,
}

impl HeaderLayout {
    /// `board` is the area of all cells, i.e. `BoardLayout::rect()`.
    pub fn above(board: Rect) -> Self {
        let h = theme::HEADER_HEIGHT;
        // Vertically centred in the margin between the window's top and the board's frame.
        let y = (board.y - theme::BEVEL - h) / 2.0;
        let led_w = theme::LED_WIDTH;

        Self {
            counter: Rect::new(board.x, y, led_w, h),
            face: Rect::new(board.center().x - h / 2.0, y, h, h),
            timer: Rect::new(board.right() - led_w, y, led_w, h),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A Beginner board: 9 x 24 px, starting at the usual margins.
    fn board() -> Rect {
        Rect::new(12.0, 60.0, 216.0, 216.0)
    }

    #[test]
    fn counter_is_left_aligned_and_timer_right_aligned_with_the_board() {
        let header = HeaderLayout::above(board());
        assert_eq!(header.counter.left(), board().left());
        assert_eq!(header.timer.right(), board().right());
    }

    #[test]
    fn face_is_centred_over_the_board_and_square() {
        let header = HeaderLayout::above(board());
        assert_eq!(header.face.center().x, board().center().x);
        assert_eq!(header.face.w, header.face.h);
    }

    #[test]
    fn everything_is_in_one_row_above_the_board_frame_without_overlapping() {
        let header = HeaderLayout::above(board());
        let parts = [header.counter, header.face, header.timer];
        for part in parts {
            assert_eq!(part.y, header.face.y, "one row: {part:?}");
            assert!(part.top() >= 0.0, "inside the window: {part:?}");
            assert!(
                part.bottom() <= board().top() - theme::BEVEL,
                "above the board's frame: {part:?}"
            );
        }
        assert!(header.counter.right() < header.face.left(), "counter and face overlap");
        assert!(header.face.right() < header.timer.left(), "face and timer overlap");
    }
}
