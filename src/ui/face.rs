use crate::logic::GameStatus;

/// The smiley on the new-game button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    /// 🙂 playing (or about to)
    Smile,
    /// 😮 the left button is held down over the board
    Surprised,
    /// 😎 won
    Cool,
    /// 😵 lost
    Dead,
}

impl Face {
    /// `pressing`: the left mouse button is currently held down over the board.
    pub fn for_game(status: GameStatus, pressing: bool) -> Self {
        match status {
            GameStatus::Won => Face::Cool,
            GameStatus::Lost => Face::Dead,
            GameStatus::Ready | GameStatus::Playing if pressing => Face::Surprised,
            GameStatus::Ready | GameStatus::Playing => Face::Smile,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_game_status_and_the_mouse() {
        let cases = [
            (GameStatus::Ready, false, Face::Smile),
            (GameStatus::Ready, true, Face::Surprised),
            (GameStatus::Playing, false, Face::Smile),
            (GameStatus::Playing, true, Face::Surprised),
            (GameStatus::Won, false, Face::Cool),
            (GameStatus::Won, true, Face::Cool), // pressing doesn't matter once it's over
            (GameStatus::Lost, false, Face::Dead),
            (GameStatus::Lost, true, Face::Dead),
        ];
        for (status, pressing, expected) in cases {
            assert_eq!(
                Face::for_game(status, pressing),
                expected,
                "{status:?}, pressing: {pressing}"
            );
        }
    }
}
