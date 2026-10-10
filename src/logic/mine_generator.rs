use rand::Rng;
use rand::seq::IndexedRandom;

use super::BoardProperties;
use super::Position;

/// Picks `properties.mines` distinct random positions on the board, never `safe`.
/// Panics (with a message containing "room") unless `mines < rows * cols`.
pub fn generate_mines(properties: BoardProperties, safe: Position, rng: &mut impl Rng) -> Vec<Position> {
    assert!(
        properties.mines < properties.rows * properties.cols,
        "There is not enough room for {} on a {}x{} board.",
        properties.mines,
        properties.rows,
        properties.cols
    );

    let candidates = (0..properties.rows)
        .flat_map(|row| (0..properties.cols).map(move |col| Position::new(row, col)))
        .filter(|&pos| pos != safe)
        .collect::<Vec<Position>>();

    candidates
        .sample(rng, properties.mines)
        .copied()
        .collect::<Vec<Position>>()
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
