use super::Board;
use super::BoardProperties;
use super::CellContent;
use super::CellState;
use super::Difficulty;
use super::GameStatus;
use super::Position;
use super::generate_mines;
use rand::rngs::StdRng;
use std::time::Instant;

pub struct Game {
    board: Board,
    status: GameStatus,
    rng: StdRng,
    properties: BoardProperties,
    started_at: Option<Instant>,
    ended_at: Option<Instant>,
}

impl Game {
    /// A new game with a truly random generator.
    pub fn new(difficulty: Difficulty) -> Self {
        Self::from_rng(difficulty, rand::make_rng())
    }

    fn from_rng(difficulty: Difficulty, rng: StdRng) -> Self {
        let properties = difficulty.board_properties();
        Self {
            board: Board::new(properties),
            status: GameStatus::default(),
            rng,
            properties,
            started_at: None,
            ended_at: None,
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
        let Some(cell) = self.board.cell(pos) else {
            return;
        };

        match (self.status, cell.state) {
            (GameStatus::Won | GameStatus::Lost, _) => return,
            (_, CellState::Flagged | CellState::Revealed) => return,
            (GameStatus::Ready, CellState::Hidden | CellState::Questioned) => {
                let mines = generate_mines(self.properties, pos, &mut self.rng);
                self.board.place_mines(&mines);
                self.status = GameStatus::Playing;
                self.started_at = Some(Instant::now());
            }
            (GameStatus::Playing, CellState::Hidden | CellState::Questioned) => {}
        }

        let Some(cell) = self.board.cell_mut(pos) else {
            return;
        };

        cell.reveal();

        match cell.content {
            CellContent::Mine => {
                self.status = GameStatus::Lost;
                self.ended_at = Some(Instant::now());
                return;
            }
            CellContent::Empty { adjacent_mines: 0 } => self.flood_reveal(pos),
            CellContent::Empty { .. } => {}
        }

        if self.all_safe_cells_revealed() {
            self.status = GameStatus::Won;
            self.ended_at = Some(Instant::now());
            self.flag_all_mines();
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

    /// Middle click (or both buttons) on an opened number: if exactly that many neighbours
    /// are flagged, open all its other neighbours. A wrong flag means you hit a mine.
    pub fn chord(&mut self, pos: Position) {
        if self.status != GameStatus::Playing {
            return;
        }
        let Some(cell) = self.board.cell(pos) else { return };
        if cell.state != CellState::Revealed {
            return;
        }
        // An opened mine has no count (and the game would already be lost anyway).
        let CellContent::Empty { adjacent_mines } = cell.content else {
            return;
        };
        let wanted = usize::from(adjacent_mines);
        let neighbours = self.board.neighbours(pos);
        let flagged_neighbor_count = neighbours
            .iter()
            .filter(|&&p| self.board.cell(p).is_some_and(|c| c.state == CellState::Flagged))
            .count();
        if wanted != flagged_neighbor_count {
            return;
        }
        for n in neighbours {
            self.reveal(n);
        }
    }

    /// For the counter at the top left: total mines minus flags. Goes negative if the player
    /// places too many flags. Question marks don't count.
    pub fn mines_left(&self) -> isize {
        let flagged: usize = self
            .board
            .cells
            .iter()
            .flatten()
            .filter(|c| c.state == CellState::Flagged)
            .count();

        let mines = isize::try_from(self.properties.mines).expect("mine count fits in isize");
        let flagged = isize::try_from(flagged).expect("flag count fits in isize");

        mines - flagged
    }

    /// For the timer at the top right: whole seconds since the first opening click,
    /// frozen once the game is won or lost, never more than 999. 0 before the first click.
    pub fn elapsed_seconds(&self, now: Instant) -> u64 {
        let Some(start) = self.started_at else { return 0 };
        let end = self.ended_at.unwrap_or(now);
        end.saturating_duration_since(start).as_secs().min(999)
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
                CellState::Hidden | CellState::Questioned => {
                    cell.reveal();
                    if cell.content == (CellContent::Empty { adjacent_mines: 0 }) {
                        to_visit.extend(self.board.neighbours(pos));
                    }
                }
            }
        }
    }

    /// Like the original: winning puts a flag on every mine, so the counter shows 0.
    fn flag_all_mines(&mut self) {
        for cell in self.board.cells.iter_mut().flatten() {
            if cell.is_mine() {
                cell.state = CellState::Flagged;
            }
        }
    }

    fn all_safe_cells_revealed(&self) -> bool {
        self.board
            .cells
            .iter()
            .flatten()
            .all(|cell| cell.is_mine() || cell.state == CellState::Revealed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::time::Duration;

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
        game.board()
            .cells
            .iter()
            .flatten()
            .filter(|cell| cell.is_mine())
            .count()
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
                        CellState::Revealed => match cell.content {
                            CellContent::Mine => '*',
                            CellContent::Empty { adjacent_mines: 0 } => '.',
                            CellContent::Empty { adjacent_mines } => char::from(b'0' + adjacent_mines),
                        },
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

    /// A new game whose mine layout is repeatable: same seed, same layout.
    /// (`from_rng` is private, but a child module like `tests` can still call it.)
    fn seeded_game(difficulty: Difficulty, seed: u64) -> Game {
        Game::from_rng(difficulty, StdRng::seed_from_u64(seed))
    }

    /// Mines already placed and status Playing, as if the first click already happened.
    fn with_mines(properties: BoardProperties, mines: &[Position]) -> Game {
        let mut board = Board::new(properties);
        board.place_mines(mines);
        Game {
            board,
            status: GameStatus::Playing,
            rng: StdRng::seed_from_u64(0),
            properties,
            // As if the first click just happened, so the timer is running.
            started_at: Some(Instant::now()),
            ended_at: None,
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
                let mut game = seeded_game(Difficulty::Beginner, seed);
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
            let mut game = seeded_game(Difficulty::Expert, 7);
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
            assert_eq!(
                picture(&game),
                ["F1.", "11.", "..."],
                "the mine gets its flag on the win"
            );
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
            assert_eq!(
                state_at(&game, p(0, 1)),
                CellState::Flagged,
                "flagged by the win, not opened"
            );
        }

        #[test]
        fn winning_flags_every_mine() {
            let mut game = walled_game(); // 5 mines across row 2
            game.toggle_mark(p(2, 0)); // already flagged: stays flagged
            game.toggle_mark(p(2, 1));
            game.toggle_mark(p(2, 1)); // a question mark: becomes a flag
            game.reveal(p(0, 0));
            game.reveal(p(4, 4));
            assert_eq!(game.status(), GameStatus::Won);
            assert_eq!(picture(&game), [".....", "23332", "FFFFF", "23332", "....."]);
            assert_eq!(game.mines_left(), 0);
        }

        #[test]
        fn losing_does_not_flag_anything() {
            let mut game = walled_game();
            game.reveal(p(2, 2));
            assert_eq!(game.status(), GameStatus::Lost);
            assert_eq!(game.mines_left(), 5);
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

    mod chording {
        use super::*;

        #[test]
        fn opens_the_other_neighbours_when_flags_match_the_number() {
            let mut game = walled_game();
            game.reveal(p(1, 0)); // a "2"
            game.toggle_mark(p(2, 0));
            game.toggle_mark(p(2, 1));
            game.chord(p(1, 0)); // opens (0,0), a 0, which floods the top
            assert_eq!(picture(&game), [".....", "23332", "FF###", "#####", "#####"]);
        }

        #[test]
        fn does_nothing_with_too_few_flags() {
            let mut game = walled_game();
            game.reveal(p(1, 0));
            game.toggle_mark(p(2, 0)); // only 1 flag around a "2"
            game.chord(p(1, 0));
            assert_eq!(picture(&game), ["#####", "2####", "F####", "#####", "#####"]);
        }

        #[test]
        fn a_wrong_flag_makes_you_lose() {
            let mut game = walled_game();
            game.reveal(p(1, 0));
            game.toggle_mark(p(2, 0));
            game.toggle_mark(p(0, 0)); // wrong: (0,0) is safe, so (2,1) is the unflagged mine
            game.chord(p(1, 0));
            assert_eq!(game.status(), GameStatus::Lost);
        }

        #[test]
        fn does_nothing_on_a_hidden_cell() {
            let mut game = walled_game();
            game.toggle_mark(p(2, 0));
            game.toggle_mark(p(2, 1));
            game.chord(p(1, 0)); // not opened yet
            assert_eq!(picture(&game), ["#####", "#####", "FF###", "#####", "#####"]);
        }

        #[test]
        fn can_win_the_game() {
            // 2x2, mine top-left: every other cell is a "1".
            let mut game = game(2, 2, &[p(0, 0)]);
            game.reveal(p(1, 1));
            game.toggle_mark(p(0, 0));
            game.chord(p(1, 1));
            assert_eq!(game.status(), GameStatus::Won);
        }

        #[test]
        fn does_nothing_before_the_first_click() {
            let mut game = Game::new(Difficulty::Beginner);
            game.chord(p(4, 4));
            assert_eq!(game.status(), GameStatus::Ready);
            assert_eq!(mine_count(&game), 0);
        }
    }

    mod mine_counter {
        use super::*;

        #[test]
        fn starts_at_the_total_before_mines_are_placed() {
            assert_eq!(Game::new(Difficulty::Beginner).mines_left(), 10);
            assert_eq!(Game::new(Difficulty::Expert).mines_left(), 99);
        }

        #[test]
        fn counts_flags_but_not_question_marks() {
            let mut game = walled_game(); // 5 mines
            game.toggle_mark(p(0, 0));
            game.toggle_mark(p(0, 1));
            game.toggle_mark(p(0, 2));
            game.toggle_mark(p(0, 2)); // flag -> question mark
            assert_eq!(game.mines_left(), 3);
        }

        #[test]
        fn goes_negative_with_too_many_flags() {
            let mut game = walled_game(); // 5 mines
            for col in 0..5 {
                game.toggle_mark(p(0, col));
            }
            game.toggle_mark(p(1, 0));
            assert_eq!(game.mines_left(), -1);
        }
    }

    /// The tests set `started_at`/`ended_at` by hand, so they never have to wait for real seconds.
    mod timer {
        use super::*;

        fn secs(s: f64) -> Duration {
            Duration::from_secs_f64(s)
        }

        #[test]
        fn is_zero_before_the_first_click() {
            let mut game = Game::new(Difficulty::Beginner);
            game.toggle_mark(p(0, 0)); // marking doesn't start it either
            assert_eq!(game.elapsed_seconds(Instant::now() + secs(10.0)), 0);
        }

        #[test]
        fn starts_on_the_first_reveal() {
            let mut game = seeded_game(Difficulty::Beginner, 1);
            assert!(game.started_at.is_none());
            game.reveal(p(4, 4));
            assert!(game.started_at.is_some(), "the first reveal must start the timer");
        }

        #[test]
        fn counts_whole_seconds() {
            let mut game = walled_game();
            let t0 = Instant::now();
            game.started_at = Some(t0);
            assert_eq!(game.elapsed_seconds(t0), 0);
            assert_eq!(game.elapsed_seconds(t0 + secs(5.9)), 5);
        }

        #[test]
        fn never_shows_more_than_999() {
            let mut game = walled_game();
            let t0 = Instant::now();
            game.started_at = Some(t0);
            assert_eq!(game.elapsed_seconds(t0 + secs(5000.0)), 999);
        }

        #[test]
        fn stops_when_the_game_is_lost() {
            let mut game = walled_game();
            game.reveal(p(2, 2)); // a mine
            assert!(game.ended_at.is_some(), "losing must stop the timer");

            let t0 = Instant::now();
            game.started_at = Some(t0);
            game.ended_at = Some(t0 + secs(7.0));
            assert_eq!(
                game.elapsed_seconds(t0 + secs(100.0)),
                7,
                "a stopped timer must not keep counting"
            );
        }

        #[test]
        fn a_finished_game_also_shows_at_most_999() {
            let mut game = walled_game();
            game.reveal(p(2, 2)); // lose
            let t0 = Instant::now();
            game.started_at = Some(t0);
            game.ended_at = Some(t0 + secs(5000.0));
            assert_eq!(game.elapsed_seconds(t0 + secs(6000.0)), 999);
        }

        #[test]
        fn stops_when_the_game_is_won() {
            let mut game = game(1, 3, &[p(0, 1)]);
            game.reveal(p(0, 0));
            assert!(game.ended_at.is_none(), "still playing");
            game.reveal(p(0, 2));
            assert!(game.ended_at.is_some(), "winning must stop the timer");
        }
    }
}
