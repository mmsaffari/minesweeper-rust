use super::{Clue, Hint, View};
use crate::logic::Position;

/// Finds a cell that is certainly safe or certainly a mine, from what's visible only.
///
/// A safe cell is preferred, because it's the more useful hint: it moves the game on.
/// The rules, each applied to every number on the board:
///
/// 1. **All found:** a number whose mines are all flagged → its other unknown neighbours are safe.
/// 2. **All unknown are mines:** a number with as many unknown neighbours as missing mines →
///    they're all mines.
/// 3. **Subset rule:** if number A's unknown cells are all also unknown cells of number B,
///    then B's *extra* cells hold exactly `B.mines − A.mines` mines. If that's 0 they're all
///    safe; if it's as many as there are extra cells, they're all mines.
///
/// Rules 1 and 2 are really rule 3 with an empty A, but are checked first because they're
/// the ones a player spots most easily.
pub fn find_hint(view: &View) -> Hint {
    let clues = Clue::all(view);
    let mut mine = None;

    for clue in &clues {
        let first = first(&clue.unknown);
        if clue.mines == 0 {
            return Hint::Safe(first);
        }
        if clue.mines == clue.unknown.len() {
            mine.get_or_insert(first);
        }
    }

    for a in &clues {
        for b in &clues {
            if a.unknown.len() >= b.unknown.len() || !a.unknown.is_subset(&b.unknown) {
                continue; // only a *smaller* set that lies completely inside b counts
            }
            let Some(extra_mines) = b.mines.checked_sub(a.mines) else {
                continue; // a needs more mines than b allows: a flag is wrong somewhere
            };
            let extra: Vec<Position> = b.unknown.difference(&a.unknown).copied().collect();
            if extra_mines == 0 {
                return Hint::Safe(extra[0]);
            }
            if extra_mines == extra.len() {
                mine.get_or_insert(extra[0]);
            }
        }
    }

    mine.map_or(Hint::NoSureMove, Hint::Mine)
}

/// A clue's unknown cells are never empty (`Clue::all` leaves those out).
fn first(cells: &std::collections::BTreeSet<Position>) -> Position {
    *cells.first().expect("a clue has unknown cells")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::{Difficulty, Game, GameStatus};
    use crate::solver::view::tests::parse;

    fn p(row: usize, col: usize) -> Position {
        Position::new(row, col)
    }

    fn hint(board: &str) -> Hint {
        find_hint(&parse(board))
    }

    // --- rule 1: all mines found ---

    #[test]
    fn a_number_with_all_its_flags_makes_the_rest_safe() {
        assert_eq!(hint("F1#"), Hint::Safe(p(0, 2)));
    }

    #[test]
    fn question_marks_count_as_unknown_not_as_flags() {
        assert_eq!(hint("F1?"), Hint::Safe(p(0, 2)));
    }

    // --- rule 2: as many unknown cells as missing mines ---

    #[test]
    fn a_number_with_exactly_that_many_unknown_cells_marks_them_as_mines() {
        assert_eq!(hint("1#"), Hint::Mine(p(0, 1)));
        assert_eq!(
            hint("F2#"),
            Hint::Mine(p(0, 2)),
            "one flag found, one mine left, one cell"
        );
    }

    // --- rule 3: subsets ---

    /// The 1-1 pattern along an edge: the left 1 sees (0,0) and (0,1); the right 1 sees
    /// those too, plus (0,2) and (1,2). Its one mine is already in the left pair, so the rest is safe.
    #[test]
    fn subset_rule_finds_a_safe_cell() {
        let board = "
            ###
            11#
        ";
        assert_eq!(hint(board), Hint::Safe(p(0, 2)));
    }

    /// Like above, but the right number is a 3 with one flag: 2 mines among its 3 unknown
    /// cells, 1 of them in the left pair, so the extra cell (0,2) must be the other.
    #[test]
    fn subset_rule_finds_a_mine() {
        let board = "
            ###
            13F
        ";
        assert_eq!(hint(board), Hint::Mine(p(0, 2)));
    }

    // --- choosing and giving up ---

    #[test]
    fn a_safe_cell_beats_a_mine() {
        // (0,1) is certainly a mine, but (0,5) is certainly safe: that's the better hint.
        assert_eq!(hint("1##F1#"), Hint::Safe(p(0, 5)));
    }

    /// The classic 50/50: one mine, two cells, and nothing else to go on.
    #[test]
    fn says_so_when_only_a_guess_is_left() {
        let board = "
            ##
            11
        ";
        assert_eq!(hint(board), Hint::NoSureMove);
    }

    #[test]
    fn a_board_without_numbers_has_no_sure_move() {
        assert_eq!(hint("###\n###"), Hint::NoSureMove);
    }

    #[test]
    fn a_number_with_too_many_flags_is_not_trusted() {
        // The 1 has two flags, so one is wrong; it says nothing about (0,3).
        assert_eq!(hint("F1F#"), Hint::NoSureMove);
    }

    // --- against real games ---

    /// Property test: play real random games using nothing but hints, and check every hint
    /// against where the mines really are. A safe hint must never be a mine, a mine hint must
    /// always be one, and following the hints must never lose the game.
    #[test]
    fn hints_are_always_right_in_real_games() {
        let games = [
            (Difficulty::Beginner, 40),
            (Difficulty::Intermediate, 15),
            (Difficulty::Expert, 5),
        ];
        for (difficulty, count) in games {
            for _ in 0..count {
                let mut game = Game::new(difficulty);
                game.reveal(p(4, 4)); // the first click, always safe

                while game.status() == GameStatus::Playing {
                    match find_hint(&View::from_game(&game)) {
                        Hint::Safe(pos) => {
                            assert!(
                                !is_mine(&game, pos),
                                "{difficulty:?}: {pos:?} isn't safe\n{}",
                                show(&game)
                            );
                            game.reveal(pos);
                        }
                        Hint::Mine(pos) => {
                            assert!(
                                is_mine(&game, pos),
                                "{difficulty:?}: {pos:?} isn't a mine\n{}",
                                show(&game)
                            );
                            game.toggle_mark(pos);
                        }
                        Hint::NoSureMove => break,
                    }
                }
                assert_ne!(game.status(), GameStatus::Lost, "following hints lost the game");
            }
        }
    }

    fn is_mine(game: &Game, pos: Position) -> bool {
        game.board().cell(pos).expect("on the board").is_mine()
    }

    fn show(game: &Game) -> String {
        View::from_game(game).to_string()
    }
}
