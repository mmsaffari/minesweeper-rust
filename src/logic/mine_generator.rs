use rand::Rng;
use rand::seq::IndexedRandom;

use super::BoardProperties;
use super::Position;

/// Picks `properties.mines` distinct random positions on the board, never `safe`.
/// Panics (with a message containing "room") unless `mines < rows * cols`.
///
/// Like the original, the layout is drawn from *every* cell, so it depends only on `rng`
/// (that is, on the game number), not on where the first click is. Only if a mine lands
/// on `safe` does that one mine move, to the first free cell counting from the top left.
pub fn generate_mines(properties: BoardProperties, safe: Position, rng: &mut impl Rng) -> Vec<Position> {
    assert!(
        properties.mines < properties.rows * properties.cols,
        "There is not enough room for {} on a {}x{} board.",
        properties.mines,
        properties.rows,
        properties.cols
    );

    let all_cells = (0..properties.rows)
        .flat_map(|row| (0..properties.cols).map(move |col| Position::new(row, col)))
        .collect::<Vec<Position>>();

    let mut mines = all_cells
        .sample(rng, properties.mines)
        .copied()
        .collect::<Vec<Position>>();

    if let Some(index) = mines.iter().position(|&mine| mine == safe) {
        let free = all_cells
            .iter()
            .copied()
            .find(|cell| *cell != safe && !mines.contains(cell))
            .expect("there are fewer mines than cells, checked above");
        mines[index] = free;
    }
    mines
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use std::collections::HashSet;

    // --- helpers ---

    fn props(rows: usize, cols: usize, mines: usize) -> BoardProperties {
        BoardProperties { rows, cols, mines }
    }

    fn p(row: usize, col: usize) -> Position {
        Position::new(row, col)
    }

    /// A seeded generator gives the same "random" numbers every run, so tests are repeatable.
    fn seeded(seed: u64) -> StdRng {
        StdRng::seed_from_u64(seed)
    }

    // --- generate_mines ---

    #[test]
    fn returns_exactly_the_requested_number_of_distinct_mines() {
        let expert = props(16, 30, 99);
        for seed in 0..50 {
            let mines = generate_mines(expert, p(0, 0), &mut seeded(seed));
            let unique: HashSet<Position> = mines.iter().copied().collect();
            assert_eq!(mines.len(), 99, "seed {seed}: wrong count");
            assert_eq!(unique.len(), 99, "seed {seed}: duplicates in {mines:?}");
        }
    }

    #[test]
    fn every_mine_is_on_the_board_and_never_on_the_safe_cell() {
        let (rows, cols) = (4, 7);
        let safe = p(2, 3);
        for seed in 0..200 {
            for mine in generate_mines(props(rows, cols, 10), safe, &mut seeded(seed)) {
                assert!(
                    mine.row < rows && mine.col < cols,
                    "seed {seed}: {mine:?} is off the board"
                );
                assert_ne!(mine, safe, "seed {seed}: mine placed on the safe cell");
            }
        }
    }

    /// 8 mines on a 3x3 board: the only valid layout is "everything except the safe cell".
    #[test]
    fn full_board_fills_every_cell_except_the_safe_one() {
        let safe = p(1, 1);
        let all_but_safe: HashSet<Position> = (0..3)
            .flat_map(|r| (0..3).map(move |c| p(r, c)))
            .filter(|&q| q != safe)
            .collect();
        for seed in 0..20 {
            let mines: HashSet<Position> = generate_mines(props(3, 3, 8), safe, &mut seeded(seed))
                .into_iter()
                .collect();
            assert_eq!(mines, all_but_safe, "seed {seed}");
        }
    }

    #[test]
    fn zero_mines_gives_an_empty_list() {
        assert!(generate_mines(props(9, 9, 0), p(4, 4), &mut seeded(1)).is_empty());
    }

    #[test]
    fn same_seed_gives_the_same_layout() {
        let beginner = props(9, 9, 10);
        assert_eq!(
            generate_mines(beginner, p(0, 0), &mut seeded(42)),
            generate_mines(beginner, p(0, 0), &mut seeded(42)),
        );
    }

    /// The layout a seed gives before any first click is taken into account
    /// (a "safe" cell off the board never matches a mine).
    fn raw_layout(seed: u64) -> Vec<Position> {
        generate_mines(props(9, 9, 10), p(99, 99), &mut seeded(seed))
    }

    /// That's what makes game numbers shareable: the same number is the same game,
    /// wherever the first click goes.
    #[test]
    fn the_layout_comes_from_the_seed_not_from_the_first_click() {
        for seed in 0..20 {
            let raw = raw_layout(seed);
            for row in 0..9 {
                for col in 0..9 {
                    let first_click = p(row, col);
                    if !raw.contains(&first_click) {
                        let mines = generate_mines(props(9, 9, 10), first_click, &mut seeded(seed));
                        assert_eq!(mines, raw, "seed {seed}, first click {first_click:?}");
                    }
                }
            }
        }
    }

    /// Like the original: only the mine under the first click moves, to the first free cell
    /// counting from the top left.
    #[test]
    fn a_mine_under_the_first_click_moves_to_the_first_free_cell() {
        for seed in 0..20 {
            let raw = raw_layout(seed);
            let first_click = raw[0];
            let first_free = (0..9)
                .flat_map(|r| (0..9).map(move |c| p(r, c)))
                .find(|&cell| cell != first_click && !raw.contains(&cell))
                .unwrap();
            let mut expected = raw.clone();
            expected[0] = first_free;
            assert_eq!(
                generate_mines(props(9, 9, 10), first_click, &mut seeded(seed)),
                expected,
                "seed {seed}"
            );
        }
    }

    /// Guards against an implementation that ignores `rng` and always returns the same cells.
    #[test]
    fn different_seeds_give_different_layouts() {
        let beginner = props(9, 9, 10);
        let layouts: HashSet<Vec<Position>> = (0..10)
            .map(|seed| generate_mines(beginner, p(0, 0), &mut seeded(seed)))
            .collect();
        assert!(layouts.len() > 1, "10 seeds all produced the same layout");
    }

    #[test]
    #[should_panic(expected = "room")]
    fn panics_when_mines_do_not_fit() {
        // 9 cells minus the safe one leaves room for 8, not 9.
        generate_mines(props(3, 3, 9), p(0, 0), &mut seeded(1));
    }
}
