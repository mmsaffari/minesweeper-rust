use std::path::PathBuf;
use std::time::Instant;

use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::math::Vec2;
use macroquad::prelude::clear_background;

use crate::logic::{Difficulty, Game, GameStatus};
use crate::scores::{self, BestTimes};
use crate::ui::{
    BoardLayout, ClickTracker, Command, Face, Fonts, HeaderLayout, Menu, MenuClick, MenuLayout, Press, any_button_down,
    draw_board, draw_header, draw_menu, face_clicked, handle_mouse, keyboard_command, left_pressed, mouse,
    pressed_cells, theme, window_size,
};

/// Everything the running program keeps between frames: which difficulty is chosen,
/// the current game, where things are on screen, and the mouse gesture in progress.
pub struct App {
    difficulty: Difficulty,
    game: Game,
    layout: BoardLayout,
    header: HeaderLayout,
    tracker: ClickTracker,
    menu: Menu,
    menu_layout: MenuLayout,
    /// The current mouse gesture started on the menu (or closed it), so until every button
    /// is up again, nothing may happen on the board or the face.
    menu_owns_mouse: bool,
    best_times: BestTimes,
    /// Where to save new records; `None` in tests (and on a system without a home folder).
    best_times_path: Option<PathBuf>,
    /// This game's win has already been checked for a record (it's won for many frames).
    win_recorded: bool,
}

impl App {
    pub fn new(difficulty: Difficulty, best_times: BestTimes, best_times_path: Option<PathBuf>) -> Self {
        let layout = BoardLayout::for_difficulty(difficulty);
        Self {
            difficulty,
            game: Game::new(difficulty),
            layout,
            header: HeaderLayout::above(layout.rect()),
            tracker: ClickTracker::default(),
            menu: Menu::default(),
            menu_layout: MenuLayout::new(window_size(layout.rect()).x),
            menu_owns_mouse: false,
            best_times,
            best_times_path,
            win_recorded: false,
        }
    }

    pub fn window_size(&self) -> Vec2 {
        window_size(self.layout.rect())
    }

    /// Reads this frame's mouse and keyboard input and updates the game.
    /// Returns the new window size when the window has to change.
    pub fn update(&mut self) -> Option<Vec2> {
        let mut command = None;

        // 1. The menu gets the first look at a click.
        if left_pressed() {
            match self.menu.click(mouse(), &self.menu_layout) {
                MenuClick::NotMine => {}
                MenuClick::Consumed => self.menu_owns_mouse = true,
                MenuClick::Chose(chosen) => {
                    self.menu_owns_mouse = true;
                    command = Some(chosen);
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.menu.close();
        }

        // 2. The board and the face, unless the menu has the mouse.
        let board_enabled = self.board_enabled();
        handle_mouse(&mut self.game, &self.layout, &mut self.tracker, board_enabled);
        self.record_win(Instant::now());
        if board_enabled && face_clicked(&self.header) {
            command = command.or(Some(Command::NewGame));
        }

        // 3. Keyboard shortcuts work whether the menu is open or not.
        command = command.or_else(keyboard_command);

        // The gesture is over once every button is up.
        if !any_button_down() {
            self.menu_owns_mouse = false;
        }

        command.and_then(|command| self.apply(command))
    }

    fn board_enabled(&self) -> bool {
        !self.menu_owns_mouse && !self.menu.is_open()
    }

    /// On the first frame after a win: remember the time if it's a record, and save it.
    /// Saving can fail (no permission, disk full); that's reported, but the game goes on.
    fn record_win(&mut self, now: Instant) {
        if self.game.status() != GameStatus::Won || self.win_recorded {
            return;
        }
        self.win_recorded = true;

        let seconds = self.game.elapsed_seconds(now);
        if self.best_times.record(self.difficulty, seconds)
            && let Some(path) = &self.best_times_path
            && let Err(error) = scores::save(path, &self.best_times)
        {
            eprintln!("Couldn't save best times to {}: {error}", path.display());
        }
    }

    /// Carries out a command. Returns the new window size if the board size changed.
    /// Kept apart from `update` (which reads real input) so tests can call it directly.
    pub fn apply(&mut self, command: Command) -> Option<Vec2> {
        match command {
            Command::NewGame => {
                self.start_new_game();
                None
            }
            Command::Start(difficulty) => {
                let resized = difficulty != self.difficulty;
                // Replace only what depends on the difficulty. Everything else must survive,
                // above all `menu_owns_mouse`: the click that chose this difficulty is still
                // held down, and its release must not reach the new board.
                self.difficulty = difficulty;
                self.layout = BoardLayout::for_difficulty(difficulty);
                self.header = HeaderLayout::above(self.layout.rect());
                self.menu_layout = MenuLayout::new(self.window_size().x);
                self.start_new_game();
                resized.then(|| self.window_size())
            }
        }
    }

    fn start_new_game(&mut self) {
        self.game = Game::new(self.difficulty);
        self.win_recorded = false;
    }

    pub fn draw(&self, fonts: &Fonts) {
        // What's held down, and where: drives both the pushed-in cells and the surprised face.
        let press = if self.board_enabled() {
            self.tracker.held()
        } else {
            Press::None
        };
        let under_mouse = self.layout.position_at(mouse());
        let pressed = pressed_cells(&self.game, under_mouse, press);
        let face = Face::for_game(self.game.status(), press != Press::None && under_mouse.is_some());
        clear_background(theme::BACKGROUND);
        draw_header(&self.game, &self.header, face, &fonts.counter);
        draw_board(&self.game, &self.layout, &pressed, &fonts.cell);
        // Last, so the open dropdown is drawn on top of the board.
        draw_menu(
            &self.menu_layout,
            self.menu.is_open(),
            self.difficulty,
            &self.best_times,
            &fonts.cell,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::{GameStatus, Position};

    /// An app that never touches the real best-times file.
    fn app(difficulty: Difficulty) -> App {
        App::new(difficulty, BestTimes::default(), None)
    }

    fn board_size(app: &App) -> (usize, usize) {
        (app.game.board().rows(), app.game.board().cols())
    }

    #[test]
    fn starts_with_a_fresh_game_of_the_given_difficulty() {
        let app = app(Difficulty::Intermediate);
        assert_eq!(app.game.status(), GameStatus::Ready);
        assert_eq!(board_size(&app), (16, 16));
    }

    #[test]
    fn new_game_restarts_without_resizing() {
        let mut app = app(Difficulty::Beginner);
        app.game.reveal(Position::new(4, 4));
        assert_ne!(app.game.status(), GameStatus::Ready);

        assert_eq!(app.apply(Command::NewGame), None, "same board, same window");
        assert_eq!(app.game.status(), GameStatus::Ready);
        assert_eq!(board_size(&app), (9, 9));
    }

    #[test]
    fn a_new_difficulty_restarts_and_resizes_the_window() {
        let mut app = app(Difficulty::Beginner);
        let beginner_window = app.window_size();

        let new_window = app.apply(Command::Start(Difficulty::Expert));

        assert_eq!(board_size(&app), (16, 30));
        assert_eq!(app.game.status(), GameStatus::Ready);
        assert_eq!(new_window, Some(app.window_size()));
        assert!(app.window_size().x > beginner_window.x, "Expert is wider than Beginner");
    }

    #[test]
    fn choosing_the_same_difficulty_restarts_without_resizing() {
        let mut app = app(Difficulty::Expert);
        app.game.reveal(Position::new(0, 0));
        assert_eq!(app.apply(Command::Start(Difficulty::Expert)), None);
        assert_eq!(app.game.status(), GameStatus::Ready);
    }

    #[test]
    fn records_survive_a_change_of_difficulty() {
        let mut app = app(Difficulty::Beginner);
        app.best_times.record(Difficulty::Beginner, 42);
        app.apply(Command::Start(Difficulty::Expert));
        assert_eq!(app.best_times.get(Difficulty::Beginner), Some(42));
    }

    /// Regression: choosing a difficulty in the menu happens on the button *press*; the
    /// *release* comes a frame later, over the new board, and must not open a cell there.
    #[test]
    fn a_menu_click_keeps_owning_the_mouse_across_a_change_of_difficulty() {
        let mut app = app(Difficulty::Beginner);
        app.menu_owns_mouse = true; // the press landed on "Expert" in the menu
        app.apply(Command::Start(Difficulty::Expert));
        assert!(!app.board_enabled(), "the release of that click would open a cell");
    }
}
