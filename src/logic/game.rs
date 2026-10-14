use super::Board;
use super::BoardProperties;
use super::CellState;
use super::Difficulty;
use super::GameStatus;
use super::Position;
use super::generate_mines;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct Game {
    board: Board,
    status: GameStatus,
    rng: StdRng,
    properties: BoardProperties,
}

impl Game {
    /// A new game with a truly random generator.
    pub fn new(difficulty: Difficulty) -> Self {
        Self::from_rng(difficulty, rand::make_rng())
    }

    /// A new game whose mine layout is repeatable: same seed, same layout.
    pub fn with_seed(difficulty: Difficulty, seed: u64) -> Self {
        Self::from_rng(difficulty, StdRng::seed_from_u64(seed))
    }

    fn from_rng(difficulty: Difficulty, rng: StdRng) -> Self {
        let properties = difficulty.board_properties();
        Self {
            board: Board::new(properties),
            status: GameStatus::default(),
            rng,
            properties,
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    /// Read-only access for the UI (and the tests) to draw the board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Left click.
    pub fn reveal(&mut self, pos: Position) {
        let Some(cell) = self.board().cell(pos) else {
            return;
        };

        if self.status() == GameStatus::Lost || self.status() == GameStatus::Won {
            return;
        }

        if cell.state == CellState::Flagged || cell.state == CellState::Revealed {
            return;
        }

        if self.status() == GameStatus::Ready {
            let mines = generate_mines(self.properties, pos, &mut self.rng);
            self.board.place_mines(&mines);
            self.status = GameStatus::Playing;
        }

        let Some(cell) = self.board.cell_mut(pos) else {
            return;
        };

        cell.reveal();

        if cell.is_mine {
            self.status = GameStatus::Lost;
            return;
        }

        if cell.adjacent_mines == 0 {
            self.flood_reveal(pos);
        }

        if self.all_safe_cells_revealed() {
            self.status = GameStatus::Won;
        }
    }

    /// Right click.
    pub fn toggle_mark(&mut self, pos: Position) {
        match self.status {
            GameStatus::Ready | GameStatus::Playing => {
                let Some(cell) = self.board.cell_mut(pos) else {
                    return;
                };
                cell.cycle_mark();
            }
            GameStatus::Won | GameStatus::Lost => {}
        };
    }

    /// Opens everything reachable from `start` (an already revealed 0) through other 0s,
    /// stopping at numbers. It never meets a mine: every neighbour of a 0 is safe by definition.
    fn flood_reveal(&mut self, start: Position) {
        let mut to_visit = self.board.neighbours(start);
        while let Some(pos) = to_visit.pop() {
            let Some(cell) = self.board.cell_mut(pos) else {
                continue;
            };
            match cell.state {
                CellState::Revealed | CellState::Flagged => continue,
                CellState::Hidden | CellState::Questioned => {}
            }
            cell.reveal();
            if cell.adjacent_mines == 0 {
                to_visit.extend(self.board.neighbours(pos));
            }
        }
    }

    fn all_safe_cells_revealed(&self) -> bool {
        self.board
            .cells
            .iter()
            .flatten()
            .all(|cell| cell.is_mine || cell.state == CellState::Revealed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- helpers ---

    fn p(row: usize, col: usize) -> Position {
        Position::new(row, col)
    }

    /// A game whose mines are already placed where we say, so outcomes are predictable.
    fn game(rows: usize, cols: usize, mines: &[Position]) -> Game {
        with_mines(
            BoardProperties {
                rows,
                cols,
                mines: mines.len(),
            },
            mines,
        )
    }

    fn state_at(game: &Game, pos: Position) -> CellState {
        game.board().cell(pos).expect("position should be on the board").state
    }

    fn mine_count(game: &Game) -> usize {
        game.board().cells.iter().flatten().filter(|cell| cell.is_mine).count()
    }

    /// The board as the player would see it, one string per row:
    /// `#` hidden, `F` flag, `?` question mark, `.` revealed 0, `1`-`8` revealed number, `*` revealed mine.
    fn picture(game: &Game) -> Vec<String> {
        game.board()
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell.state {
                        CellState::Hidden => '#',
                        CellState::Flagged => 'F',
                        CellState::Questioned => '?',
                        CellState::Revealed if cell.is_mine => '*',
                        CellState::Revealed if cell.adjacent_mines == 0 => '.',
                        CellState::Revealed => char::from(b'0' + cell.adjacent_mines),
                    })
                    .collect()
            })
            .collect()
    }

    /// 5x5 with a full row of mines across row 2, so a flood from the top can't get past it.
    ///
    /// ```text
    /// . . . . .
    /// 2 3 3 3 2
    /// * * * * *
    /// 2 3 3 3 2
    /// . . . . .
    /// ```
    fn walled_game() -> Game {
        game(5, 5, &[p(2, 0), p(2, 1), p(2, 2), p(2, 3), p(2, 4)])
    }

    /// Mines already placed and status Playing, as if the first click already happened.
    fn with_mines(properties: BoardProperties, mines: &[Position]) -> Game {
        let mut arbitary_board = Board::new(properties);
        arbitary_board.place_mines(mines);
        Game {
            board: arbitary_board,
            status: GameStatus::Playing,
            rng: StdRng::seed_from_u64(0),
            properties,
        }
    }

    /// Prints `picture` with row and column numbers, for debugging a test.
    /// Output shows only when the test fails, or with `cargo test -- --nocapture`.
    #[allow(dead_code)]
    fn print_board(game: &Game) {
        let picture = picture(game);
        let cols = picture.first().map_or(0, |row| row.len());

        let header: Vec<String> = (0..cols).map(|col| format!("{col:>2}")).collect();
        println!("   {}", header.join(" "));
        for (row, line) in picture.iter().enumerate() {
            let cells: Vec<String> = line.chars().map(|c| format!("{c:>2}")).collect();
            println!("{row:>2} {}", cells.join(" "));
        }
    }

    mod first_click {
        use super::*;

        #[test]
        fn new_game_is_ready_and_has_no_mines_yet() {
            let game = Game::new(Difficulty::Beginner);
            assert_eq!(game.status(), GameStatus::Ready);
            assert_eq!(mine_count(&game), 0, "mines are placed on the first click, not before");
        }

        #[test]
        fn first_reveal_places_all_mines_but_never_under_the_click() {
            for seed in 0..50 {
                let mut game = Game::with_seed(Difficulty::Beginner, seed);
                game.reveal(p(4, 4));
                assert_eq!(mine_count(&game), 10, "seed {seed}");
                assert_eq!(state_at(&game, p(4, 4)), CellState::Revealed, "seed {seed}");
                assert_ne!(
                    game.status(),
                    GameStatus::Lost,
                    "seed {seed}: the first click must never lose"
                );
            }
        }

        #[test]
        fn first_reveal_starts_the_game() {
            let mut game = Game::with_seed(Difficulty::Expert, 7);
            game.reveal(p(0, 0));
            assert_eq!(game.status(), GameStatus::Playing);
        }

        #[test]
        fn first_click_on_a_flag_does_not_start_the_game() {
            let mut game = Game::new(Difficulty::Beginner);
            game.toggle_mark(p(0, 0));
            game.reveal(p(0, 0));
            assert_eq!(
                state_at(&game, p(0, 0)),
                CellState::Flagged,
                "the flag must protect the cell"
            );
            assert_eq!(
                game.status(),
                GameStatus::Ready,
                "a click that did nothing must not start the game"
            );
            assert_eq!(mine_count(&game), 0, "a click that did nothing must not place mines");
        }
    }

    mod reveal {
        use super::*;

        #[test]
        fn clicking_a_question_mark_reveals_it() {
            let mut game = walled_game();
            game.toggle_mark(p(1, 2));
            game.toggle_mark(p(1, 2)); // flag -> question mark
            game.reveal(p(1, 2));
            assert_eq!(picture(&game), ["#####", "##3##", "#####", "#####", "#####"]);
        }

        /// The flood's edge is made of numbers; a `?` there must open like any other cell.
        #[test]
        fn flood_opens_question_marks_on_numbers_too() {
            let mut game = walled_game();
            game.toggle_mark(p(1, 4));
            game.toggle_mark(p(1, 4)); // flag -> question mark, on a "2"
            game.reveal(p(0, 0));
            assert_eq!(picture(&game), [".....", "23332", "#####", "#####", "#####"]);
        }

        #[test]
        fn revealing_a_number_reveals_only_that_cell() {
            let mut game = walled_game();
            game.reveal(p(1, 2));
            assert_eq!(picture(&game), ["#####", "##3##", "#####", "#####", "#####"]);
            assert_eq!(game.status(), GameStatus::Playing);
        }

        #[test]
        fn revealing_a_zero_floods_until_the_numbers() {
            let mut game = walled_game();
            game.reveal(p(0, 0));
            assert_eq!(picture(&game), [".....", "23332", "#####", "#####", "#####"]);
        }

        #[test]
        fn flood_leaves_flags_alone() {
            let mut game = walled_game();
            game.toggle_mark(p(0, 4));
            game.reveal(p(0, 0));
            assert_eq!(picture(&game), ["....F", "23332", "#####", "#####", "#####"]);
        }

        #[test]
        fn flood_opens_question_marks() {
            let mut game = walled_game();
            game.toggle_mark(p(0, 4));
            game.toggle_mark(p(0, 4)); // flag -> question mark
            game.reveal(p(0, 0));
            assert_eq!(picture(&game), [".....", "23332", "#####", "#####", "#####"]);
        }

        #[test]
        fn revealing_a_flagged_cell_does_nothing() {
            let mut game = walled_game();
            game.toggle_mark(p(0, 0));
            game.reveal(p(0, 0));
            assert_eq!(state_at(&game, p(0, 0)), CellState::Flagged);
        }

        #[test]
        fn revealing_off_the_board_is_ignored() {
            let mut game = walled_game();
            game.reveal(p(5, 5));
            game.toggle_mark(p(99, 0));
            assert_eq!(picture(&game), ["#####"; 5]);
        }
    }

    mod losing {
        use super::*;

        #[test]
        fn revealing_a_mine_loses() {
            let mut game = walled_game();
            game.reveal(p(2, 2));
            assert_eq!(game.status(), GameStatus::Lost);
            assert_eq!(state_at(&game, p(2, 2)), CellState::Revealed);
        }

        #[test]
        fn nothing_changes_after_losing() {
            let mut game = walled_game();
            game.reveal(p(2, 2));
            let before = picture(&game);
            game.reveal(p(0, 0));
            game.toggle_mark(p(4, 4));
            assert_eq!(picture(&game), before);
            assert_eq!(game.status(), GameStatus::Lost);
        }
    }

    mod winning {
        use super::*;

        #[test]
        fn revealing_every_safe_cell_wins() {
            // 1x3 with the mine in the middle: "1*1". Two clicks, no flood.
            let mut game = game(1, 3, &[p(0, 1)]);
            game.reveal(p(0, 0));
            assert_eq!(game.status(), GameStatus::Playing, "one safe cell is still hidden");
            game.reveal(p(0, 2));
            assert_eq!(game.status(), GameStatus::Won);
        }

        #[test]
        fn a_flood_can_win_in_one_click() {
            let mut game = game(3, 3, &[p(0, 0)]);
            game.reveal(p(2, 2));
            assert_eq!(picture(&game), ["#1.", "11.", "..."]);
            assert_eq!(game.status(), GameStatus::Won);
        }

        #[test]
        fn winning_does_not_require_flags() {
            let mut game = walled_game();
            game.reveal(p(0, 0));
            game.reveal(p(4, 4));
            assert_eq!(game.status(), GameStatus::Won);
        }

        #[test]
        fn nothing_changes_after_winning() {
            let mut game = game(1, 3, &[p(0, 1)]);
            game.reveal(p(0, 0));
            game.reveal(p(0, 2));
            game.reveal(p(0, 1)); // the mine — must be ignored now
            assert_eq!(game.status(), GameStatus::Won);
            assert_eq!(state_at(&game, p(0, 1)), CellState::Hidden);
        }
    }

    mod marks {
        use super::*;

        #[test]
        fn toggle_mark_cycles_a_hidden_cell() {
            let mut game = walled_game();
            game.toggle_mark(p(0, 0));
            assert_eq!(state_at(&game, p(0, 0)), CellState::Flagged);
            game.toggle_mark(p(0, 0));
            assert_eq!(state_at(&game, p(0, 0)), CellState::Questioned);
            game.toggle_mark(p(0, 0));
            assert_eq!(state_at(&game, p(0, 0)), CellState::Hidden);
        }

        #[test]
        fn marking_does_not_start_the_game() {
            let mut game = Game::new(Difficulty::Beginner);
            game.toggle_mark(p(0, 0));
            assert_eq!(game.status(), GameStatus::Ready);
            assert_eq!(mine_count(&game), 0);
        }
    }
}
