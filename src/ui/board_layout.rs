use macroquad::math::{Rect, Vec2};

use crate::logic::Position;

/// Where the board sits in the window, in pixels.
/// Drawing and mouse clicks both use this, so they can never disagree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardLayout {
    origin: Vec2,
    cell_size: f32,
    rows: usize,
    cols: usize,
}

impl BoardLayout {
    /// `origin` is the top-left corner of cell (0, 0).
    pub fn new(origin: Vec2, cell_size: f32, rows: usize, cols: usize) -> Self {
        BoardLayout {
            origin,
            cell_size,
            rows,
            cols,
        }
    }

    /// Width and height of the whole board.
    pub fn size(&self) -> Vec2 {
        // Columns run along x (width), rows along y (height).
        Vec2::new(self.cols as f32, self.rows as f32) * self.cell_size
    }

    /// The area covered by all cells together.
    pub fn rect(&self) -> Rect {
        let size = self.size();
        Rect::new(self.origin.x, self.origin.y, size.x, size.y)
    }

    /// The square a cell is drawn in.
    pub fn cell_rect(&self, pos: Position) -> Rect {
        let top_left = self.origin + Vec2::new(pos.col as f32, pos.row as f32) * self.cell_size;
        Rect::new(top_left.x, top_left.y, self.cell_size, self.cell_size)
    }

    /// Which cell a pixel belongs to, or `None` if it's outside the board.
    /// A pixel exactly on the line between two cells belongs to the right/lower one.
    pub fn position_at(&self, point: Vec2) -> Option<Position> {
        // Pixels measured from the board's top-left corner instead of the window's.
        let local = point - self.origin;

        // Left of or above the board. This check must come before the conversion below:
        // `-0.5 as usize` is 0, so a click just left of the board would land in column 0.
        if local.x < 0.0 || local.y < 0.0 {
            return None;
        }

        // Which cell, counting whole cells: 29.9 px / 20 px = 1.495 -> cell 1.
        // `as usize` drops the fraction (rounds towards zero), which is what we want here,
        // since we already know the number isn't negative.
        let col = (local.x / self.cell_size) as usize;
        let row = (local.y / self.cell_size) as usize;

        // Right of or below the board.
        if row >= self.rows || col >= self.cols {
            return None;
        }

        Some(Position::new(row, col))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- helpers ---

    fn p(row: usize, col: usize) -> Position {
        Position::new(row, col)
    }

    /// 3 rows x 4 cols of 20 px cells, top-left at (10, 60):
    ///
    /// ```text
    ///  x: 10   30   50   70   90
    /// y:60 +----+----+----+----+
    ///      |0,0 |0,1 |0,2 |0,3 |
    ///   80 +----+----+----+----+
    ///      |1,0 |    |    |    |
    ///  100 +----+----+----+----+
    ///      |2,0 |    |    |2,3 |
    ///  120 +----+----+----+----+
    /// ```
    fn layout() -> BoardLayout {
        BoardLayout::new(Vec2::new(10.0, 60.0), 20.0, 3, 4)
    }

    // --- size / cell_rect ---

    #[test]
    fn size_is_cols_by_rows_cells() {
        assert_eq!(layout().size(), Vec2::new(80.0, 60.0));
    }

    #[test]
    fn rect_covers_the_whole_board() {
        assert_eq!(layout().rect(), Rect::new(10.0, 60.0, 80.0, 60.0));
    }

    #[test]
    fn cell_rects_start_at_the_origin_and_step_by_cell_size() {
        let layout = layout();
        assert_eq!(layout.cell_rect(p(0, 0)), Rect::new(10.0, 60.0, 20.0, 20.0));
        assert_eq!(
            layout.cell_rect(p(0, 1)),
            Rect::new(30.0, 60.0, 20.0, 20.0),
            "columns go right (x)"
        );
        assert_eq!(
            layout.cell_rect(p(1, 0)),
            Rect::new(10.0, 80.0, 20.0, 20.0),
            "rows go down (y)"
        );
        assert_eq!(layout.cell_rect(p(2, 3)), Rect::new(70.0, 100.0, 20.0, 20.0));
    }

    // --- position_at ---

    #[test]
    fn points_inside_a_cell_find_that_cell() {
        let layout = layout();
        let cases = [
            ((10.0, 60.0), p(0, 0)),  // exact top-left corner of the board
            ((20.0, 70.0), p(0, 0)),  // middle of the first cell
            ((29.9, 79.9), p(0, 0)),  // just before the next cell
            ((30.0, 60.0), p(0, 1)),  // on the line: belongs to the right cell
            ((10.0, 80.0), p(1, 0)),  // on the line: belongs to the lower cell
            ((89.9, 119.9), p(2, 3)), // last pixel of the board
        ];
        for ((x, y), expected) in cases {
            assert_eq!(layout.position_at(Vec2::new(x, y)), Some(expected), "point ({x}, {y})");
        }
    }

    #[test]
    fn points_outside_the_board_find_nothing() {
        let layout = layout();
        let cases = [
            (9.9, 60.0),   // just left
            (10.0, 59.9),  // just above
            (90.0, 60.0),  // just right
            (10.0, 120.0), // just below
            (0.0, 0.0),    // the header area
            (-5.0, -5.0),  // outside the window: macroquad can report negatives while dragging
            (1e9, 1e9),    // absurdly far
        ];
        for (x, y) in cases {
            assert_eq!(layout.position_at(Vec2::new(x, y)), None, "point ({x}, {y})");
        }
    }

    /// Property test: the centre of every cell's rectangle maps back to that cell.
    #[test]
    fn every_cell_centre_maps_back_to_its_cell() {
        let layout = layout();
        for row in 0..3 {
            for col in 0..4 {
                let pos = p(row, col);
                assert_eq!(layout.position_at(layout.cell_rect(pos).center()), Some(pos), "{pos:?}");
            }
        }
    }
}
