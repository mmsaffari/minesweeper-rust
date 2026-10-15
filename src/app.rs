use macroquad::math::Vec2;
use macroquad::prelude::clear_background;

use crate::logic::{Difficulty, Game};
use crate::ui::{
    BoardLayout, ClickTracker, Command, Face, Fonts, HeaderLayout, draw_board, draw_header, face_clicked, handle_mouse,
    keyboard_command, pressing_board, theme, window_size,
};

/// Everything the running program keeps between frames: which difficulty is chosen,
/// the current game, where things are on screen, and the mouse gesture in progress.
pub struct App {
    difficulty: Difficulty,
    game: Game,
    layout: BoardLayout,
    header: HeaderLayout,
    tracker: ClickTracker,
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
        }
    }

    pub fn window_size(&self) -> Vec2 {
        window_size(self.layout.rect())
    }

    /// Reads this frame's mouse and keyboard input and updates the game.
    /// Returns the new window size when the window has to change.
    pub fn update(&mut self) -> Option<Vec2> {
        // The tracker must see every button event, including clicks on the face,
        // or it would think a button is still held down.
        handle_mouse(&mut self.game, &self.layout, &mut self.tracker);

        let command = keyboard_command().or_else(|| face_clicked(&self.header).then_some(Command::NewGame));
        command.and_then(|command| self.apply(command))
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
        let face = Face::for_game(self.game.status(), pressing_board(&self.layout));
        clear_background(theme::BACKGROUND);
        draw_header(&self.game, &self.header, face, &fonts.counter);
        draw_board(&self.game, &self.layout, &fonts.cell);
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
