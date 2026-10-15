use std::path::PathBuf;
use std::time::Instant;

use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::math::Vec2;
use macroquad::miniquad::window::clipboard_set;
use macroquad::prelude::{clear_background, get_time};

use crate::logic::{Difficulty, Game, GameStatus};
use crate::scores::{self, BestTimes};
use crate::solver::{Hint, View, find_hint};
use crate::ui::{
    AboutDialog, AboutLayout, BoardLayout, ClickTracker, Command, DialogButton, Face, Fonts, HeaderLayout, Menu,
    MenuClick, MenuLayout, NumberDialog, NumberDialogLayout, Press, Toast, any_button_down, any_button_pressed,
    draw_about, draw_board, draw_header, draw_hint, draw_menu, draw_number_dialog, draw_toast, face_clicked,
    handle_mouse, keyboard_command, left_pressed, left_released, mouse, pressed_cells, theme, typed_chars, window_size,
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
    about: AboutDialog,
    number_dialog: NumberDialog,
    /// The current mouse gesture started on the menu or a dialog (or closed one), so until
    /// every button is up again, nothing may happen on the board or the face.
    mouse_captured: bool,
    best_times: BestTimes,
    /// Where to save new records; `None` in tests (and on a system without a home folder).
    best_times_path: Option<PathBuf>,
    /// This game's win has already been checked for a record (it's won for many frames).
    win_recorded: bool,
    /// Whether this game can set a best time: only games with a fresh random number can.
    /// A replayed or chosen number may be a layout the player already knows.
    ranked: bool,
    /// A short message over the board, if one is showing.
    toast: Option<Toast>,
    /// The hint being shown (only `Safe` or `Mine`), until the next click on the board.
    hint: Option<Hint>,
    /// Text waiting to be put on the clipboard. `apply` only asks for it; `update` does it,
    /// because the clipboard needs the real window (and `apply` must work in tests).
    clipboard_request: Option<String>,
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
            about: AboutDialog::default(),
            number_dialog: NumberDialog::default(),
            mouse_captured: false,
            best_times,
            best_times_path,
            win_recorded: false,
            ranked: true,
            toast: None,
            hint: None,
            clipboard_request: None,
        }
    }

    pub fn window_size(&self) -> Vec2 {
        window_size(self.layout.rect())
    }

    /// Reads this frame's mouse and keyboard input and updates the game.
    /// Returns the new window size when the window has to change.
    pub fn update(&mut self) -> Option<Vec2> {
        if self.about.is_open() {
            self.update_about();
            return None;
        }
        if self.number_dialog.is_open() {
            return self.update_number_dialog();
        }

        let mut command = None;

        // 1. The menu gets the first look at a click.
        if left_pressed() {
            match self.menu.click(mouse(), &self.menu_layout) {
                MenuClick::NotMine => {}
                MenuClick::Consumed => self.mouse_captured = true,
                MenuClick::Chose(chosen) => {
                    self.mouse_captured = true;
                    command = Some(chosen);
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.menu.close();
        }

        // 2. The board and the face, unless the menu has the mouse.
        let board_enabled = self.board_enabled();
        if board_enabled && any_button_pressed() {
            self.hint = None; // the player is acting on the board: the hint has done its job
        }
        handle_mouse(&mut self.game, &self.layout, &mut self.tracker, board_enabled);
        self.record_win(Instant::now());
        if board_enabled && face_clicked(&self.header) {
            command = command.or(Some(Command::NewGame));
        }

        // 3. Keyboard shortcuts work whether the menu is open or not.
        command = command.or_else(keyboard_command);

        self.release_mouse_when_buttons_are_up();
        let resize = command.and_then(|command| self.apply(command));
        // Nothing reads typed characters here, but they must not pile up for the next dialog.
        typed_chars();

        // The effects `apply` can only ask for: they need the real window and clock.
        if let Some(text) = self.clipboard_request.take() {
            clipboard_set(&text);
        }
        if let Some(toast) = &mut self.toast {
            toast.start(get_time());
        }
        resize
    }

    /// While the About dialog is open it is modal: every click and key belongs to it.
    fn update_about(&mut self) {
        let layout = self.about_layout();
        if left_pressed() {
            self.mouse_captured = true;
            self.about.press(mouse(), &layout);
        }
        if left_released() {
            self.about.release(mouse(), &layout);
        }
        if [KeyCode::Escape, KeyCode::Enter, KeyCode::KpEnter]
            .into_iter()
            .any(is_key_pressed)
        {
            self.about.close();
        }
        // The tracker must still see every button event, but nothing reaches the board.
        handle_mouse(&mut self.game, &self.layout, &mut self.tracker, false);
        self.release_mouse_when_buttons_are_up();
        typed_chars();
    }

    /// While the "Choose game" dialog is open it is modal too: clicks, digits, Backspace,
    /// Enter (OK) and Esc (Cancel) all belong to it.
    fn update_number_dialog(&mut self) -> Option<Vec2> {
        let layout = self.number_dialog_layout();
        let mut clicked = None;
        if left_pressed() {
            self.mouse_captured = true;
            self.number_dialog.press(mouse(), &layout);
        }
        if left_released() {
            clicked = self.number_dialog.release(mouse(), &layout);
        }
        for c in typed_chars() {
            self.number_dialog.type_char(c);
        }
        if is_key_pressed(KeyCode::Backspace) {
            self.number_dialog.backspace();
        }
        if [KeyCode::Enter, KeyCode::KpEnter].into_iter().any(is_key_pressed) {
            clicked = Some(DialogButton::Ok);
        }
        if is_key_pressed(KeyCode::Escape) {
            clicked = Some(DialogButton::Cancel);
        }
        handle_mouse(&mut self.game, &self.layout, &mut self.tracker, false);
        self.release_mouse_when_buttons_are_up();

        match clicked {
            // An invalid number keeps the dialog open; the field shows it in red.
            Some(DialogButton::Ok) => self.number_dialog.number().and_then(|number| {
                self.number_dialog.close();
                self.apply(Command::StartNumbered(number))
            }),
            Some(DialogButton::Cancel) => {
                self.number_dialog.close();
                None
            }
            None => None,
        }
    }

    /// The gesture is over once every button is up.
    fn release_mouse_when_buttons_are_up(&mut self) {
        if !any_button_down() {
            self.mouse_captured = false;
        }
    }

    fn board_enabled(&self) -> bool {
        !self.mouse_captured && !self.menu.is_open() && !self.about.is_open() && !self.number_dialog.is_open()
    }

    fn about_layout(&self) -> AboutLayout {
        AboutLayout::centered_in(self.window_size())
    }

    fn number_dialog_layout(&self) -> NumberDialogLayout {
        NumberDialogLayout::centered_in(self.window_size())
    }

    /// On the first frame after a win: remember the time if it's a record, and save it.
    /// Saving can fail (no permission, disk full); that's reported, but the game goes on.
    fn record_win(&mut self, now: Instant) {
        if self.game.status() != GameStatus::Won || self.win_recorded || !self.ranked {
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
                self.start_game(Game::new(self.difficulty), true);
                None
            }
            Command::Replay => {
                let number = self.game.number();
                self.start_game(Game::numbered(self.difficulty, number), false);
                self.toast = Some(Toast::new("Replay: best times off"));
                None
            }
            Command::ChooseGame => {
                self.menu.close();
                self.number_dialog.show(self.game.number());
                None
            }
            Command::StartNumbered(number) => {
                self.start_game(Game::numbered(self.difficulty, number), false);
                self.toast = Some(Toast::new(format!("Game #{number}: best times off")));
                None
            }
            Command::Start(difficulty) => {
                let resized = difficulty != self.difficulty;
                // Replace only what depends on the difficulty. Everything else must survive,
                // above all `mouse_captured`: the click that chose this difficulty is still
                // held down, and its release must not reach the new board.
                self.difficulty = difficulty;
                self.layout = BoardLayout::for_difficulty(difficulty);
                self.header = HeaderLayout::above(self.layout.rect());
                self.menu_layout = MenuLayout::new(self.window_size().x);
                self.start_game(Game::new(difficulty), true);
                resized.then(|| self.window_size())
            }
            Command::About => {
                self.menu.close();
                self.about.show();
                None
            }
            Command::CopyBoard => {
                self.clipboard_request = Some(View::from_game(&self.game).to_string());
                self.toast = Some(Toast::new("Board copied"));
                None
            }
            Command::Hint => {
                self.hint = None;
                let message = match self.game.status() {
                    GameStatus::Ready => Some("First click is always safe"),
                    GameStatus::Won | GameStatus::Lost => Some("Game over (F2: new game)"),
                    GameStatus::Playing => match find_hint(&View::from_game(&self.game)) {
                        Hint::NoSureMove => Some("No sure move: guess"),
                        hint => {
                            self.hint = Some(hint);
                            None
                        }
                    },
                };
                self.toast = message.map(Toast::new);
                None
            }
        }
    }

    /// `ranked`: whether a win can set a best time (see the `ranked` field).
    fn start_game(&mut self, game: Game, ranked: bool) {
        self.game = game;
        self.ranked = ranked;
        self.win_recorded = false;
        self.hint = None;
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
        match self.hint {
            Some(Hint::Safe(pos)) => draw_hint(self.layout.cell_rect(pos), true, get_time()),
            Some(Hint::Mine(pos)) => draw_hint(self.layout.cell_rect(pos), false, get_time()),
            Some(Hint::NoSureMove) | None => {}
        }
        // After the board and the hint, so an open dropdown is drawn on top of them.
        draw_menu(
            &self.menu_layout,
            self.menu.open_menu(),
            self.difficulty,
            self.game.number(),
            &self.best_times,
            &fonts.cell,
        );
        if let Some(toast) = &self.toast
            && toast.is_visible(get_time())
        {
            draw_toast(toast.text(), self.layout.rect(), &fonts.cell);
        }
        if self.about.is_open() {
            draw_about(&self.about_layout(), self.about.ok_pressed(), &fonts.cell, get_time());
        }
        if self.number_dialog.is_open() {
            draw_number_dialog(
                &self.number_dialog,
                &self.number_dialog_layout(),
                &fonts.cell,
                get_time(),
            );
        }
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
        app.mouse_captured = true; // the press landed on "Expert" in the menu
        app.apply(Command::Start(Difficulty::Expert));
        assert!(!app.board_enabled(), "the release of that click would open a cell");
    }

    #[test]
    fn about_opens_a_modal_dialog_and_closes_the_menu() {
        let mut app = app(Difficulty::Beginner);
        app.menu
            .click(app.menu_layout.menus[1].title_rect.center(), &app.menu_layout); // open Help
        assert!(app.menu.is_open());

        assert_eq!(app.apply(Command::About), None, "no resize");
        assert!(app.about.is_open());
        assert!(!app.menu.is_open(), "the menu closes when the dialog opens");
        assert!(
            !app.board_enabled(),
            "while the dialog is open, the board ignores the mouse"
        );
    }

    /// Clicking OK closes the dialog on the *release*, but the press set `mouse_captured`,
    /// so the board stays disabled for the rest of that click.
    #[test]
    fn the_click_that_closes_the_dialog_does_not_reach_the_board() {
        let mut app = app(Difficulty::Beginner);
        app.apply(Command::About);
        let ok = app.about_layout().ok_button.center();

        app.mouse_captured = true; // what `update_about` does on the press
        app.about.press(ok, &app.about_layout());
        app.about.release(ok, &app.about_layout());

        assert!(!app.about.is_open());
        assert!(
            !app.board_enabled(),
            "until every button is up, the board must not react"
        );
    }

    #[test]
    fn copy_board_asks_for_the_visible_board_on_the_clipboard() {
        let mut app = app(Difficulty::Beginner);
        app.game.toggle_mark(Position::new(0, 0));
        assert_eq!(app.apply(Command::CopyBoard), None);

        let text = app.clipboard_request.as_deref().expect("a clipboard request");
        assert_eq!(text, View::from_game(&app.game).to_string());
        assert!(text.starts_with('F'), "{text}");
        assert!(app.toast.is_some(), "the player is told it worked");
    }

    #[test]
    fn a_hint_before_the_first_click_just_says_any_cell_is_safe() {
        let mut app = app(Difficulty::Beginner);
        app.apply(Command::Hint);
        assert_eq!(app.hint, None);
        assert!(app.toast.is_some());
    }

    #[test]
    fn a_hint_while_playing_is_right_or_admits_there_is_none() {
        for _ in 0..20 {
            let mut app = app(Difficulty::Beginner);
            app.game.reveal(Position::new(4, 4));
            if app.game.status() != GameStatus::Playing {
                continue; // the first click happened to win the whole board
            }
            app.apply(Command::Hint);
            match app.hint {
                Some(Hint::Safe(pos)) => assert!(!app.game.board().cell(pos).unwrap().is_mine()),
                Some(Hint::Mine(pos)) => assert!(app.game.board().cell(pos).unwrap().is_mine()),
                Some(Hint::NoSureMove) => panic!("NoSureMove is told as a toast, not shown as a hint"),
                None => assert!(app.toast.is_some(), "no hint, so the player is told why"),
            }
        }
    }

    #[test]
    fn a_new_game_clears_the_hint() {
        let mut app = app(Difficulty::Beginner);
        app.hint = Some(Hint::Safe(Position::new(0, 0)));
        app.apply(Command::NewGame);
        assert_eq!(app.hint, None);
    }

    #[test]
    fn replay_plays_the_same_number_without_best_times() {
        let mut app = app(Difficulty::Beginner);
        let number = app.game.number();
        app.game.reveal(Position::new(4, 4));

        assert_eq!(app.apply(Command::Replay), None);
        assert_eq!(app.game.number(), number);
        assert_eq!(app.game.status(), GameStatus::Ready, "a fresh start of the same game");
        assert!(!app.ranked, "a known layout can't set a record");
        assert!(app.toast.is_some(), "the player is told");
    }

    #[test]
    fn a_new_game_after_a_replay_counts_for_best_times_again() {
        let mut app = app(Difficulty::Beginner);
        app.apply(Command::Replay);
        app.apply(Command::NewGame);
        assert!(app.ranked);
        app.apply(Command::Replay);
        app.apply(Command::Start(Difficulty::Expert));
        assert!(app.ranked);
    }

    #[test]
    fn choose_game_opens_the_dialog_with_the_current_number() {
        let mut app = app(Difficulty::Beginner);
        app.apply(Command::ChooseGame);
        assert!(app.number_dialog.is_open());
        assert_eq!(app.number_dialog.number(), Some(app.game.number()));
        assert!(!app.board_enabled(), "the dialog is modal");
    }

    #[test]
    fn a_chosen_number_starts_that_game_without_best_times() {
        let mut app = app(Difficulty::Intermediate);
        assert_eq!(
            app.apply(Command::StartNumbered(482_913)),
            None,
            "same difficulty, same window"
        );
        assert_eq!(app.game.number(), 482_913);
        assert_eq!(board_size(&app), (16, 16));
        assert!(!app.ranked);
    }
}
