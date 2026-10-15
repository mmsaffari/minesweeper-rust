use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::math::Vec2;
use macroquad::prelude::clear_background;

use crate::logic::{Difficulty, Game};
use crate::ui::{
    BoardLayout, ClickTracker, Command, Face, Fonts, HeaderLayout, Menu, MenuClick, MenuLayout, any_button_down,
    draw_board, draw_header, draw_menu, face_clicked, handle_mouse, keyboard_command, left_pressed, mouse,
    pressing_board, theme, window_size,
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
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
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

    /// Carries out a command. Returns the new window size if the board size changed.
    /// Kept apart from `update` (which reads real input) so tests can call it directly.
    pub fn apply(&mut self, command: Command) -> Option<Vec2> {
        match command {
            Command::NewGame => {
                self.game = Game::new(self.difficulty);
                None
            }
            Command::Start(difficulty) => {
                let resized = difficulty != self.difficulty;
                *self = App::new(difficulty);
                resized.then(|| self.window_size())
            }
        }
    }

    pub fn draw(&self, fonts: &Fonts) {
        let pressing = self.board_enabled() && pressing_board(&self.layout);
        let face = Face::for_game(self.game.status(), pressing);
        clear_background(theme::BACKGROUND);
        draw_header(&self.game, &self.header, face, &fonts.counter);
        draw_board(&self.game, &self.layout, &fonts.cell);
        // Last, so the open dropdown is drawn on top of the board.
        draw_menu(&self.menu_layout, self.menu.is_open(), self.difficulty, &fonts.cell);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::{GameStatus, Position};

    fn board_size(app: &App) -> (usize, usize) {
        (app.game.board().rows(), app.game.board().cols())
    }

    #[test]
    fn starts_with_a_fresh_game_of_the_given_difficulty() {
        let app = App::new(Difficulty::Intermediate);
        assert_eq!(app.game.status(), GameStatus::Ready);
        assert_eq!(board_size(&app), (16, 16));
    }

    #[test]
    fn new_game_restarts_without_resizing() {
        let mut app = App::new(Difficulty::Beginner);
        app.game.reveal(Position::new(4, 4));
        assert_ne!(app.game.status(), GameStatus::Ready);

        assert_eq!(app.apply(Command::NewGame), None, "same board, same window");
        assert_eq!(app.game.status(), GameStatus::Ready);
        assert_eq!(board_size(&app), (9, 9));
    }

    #[test]
    fn a_new_difficulty_restarts_and_resizes_the_window() {
        let mut app = App::new(Difficulty::Beginner);
        let beginner_window = app.window_size();

        let new_window = app.apply(Command::Start(Difficulty::Expert));

        assert_eq!(board_size(&app), (16, 30));
        assert_eq!(app.game.status(), GameStatus::Ready);
        assert_eq!(new_window, Some(app.window_size()));
        assert!(app.window_size().x > beginner_window.x, "Expert is wider than Beginner");
    }

    #[test]
    fn choosing_the_same_difficulty_restarts_without_resizing() {
        let mut app = App::new(Difficulty::Expert);
        app.game.reveal(Position::new(0, 0));
        assert_eq!(app.apply(Command::Start(Difficulty::Expert)), None);
        assert_eq!(app.game.status(), GameStatus::Ready);
    }
}
