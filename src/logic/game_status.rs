#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameStatus {
    /// No cell revealed yet: mines aren't placed and the timer hasn't started.
    #[default]
    Ready,
    Playing,
    Won,
    Lost,
}

impl std::fmt::Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ready => f.pad("ready"),
            Self::Playing => f.pad("playing"),
            Self::Won => f.pad("won"),
            Self::Lost => f.pad("lost"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_ready() {
        assert_eq!(GameStatus::default(), GameStatus::Ready);
    }

    #[test]
    fn display_shows_lowercase_name() {
        let cases = [
            (GameStatus::Ready, "ready"),
            (GameStatus::Playing, "playing"),
            (GameStatus::Won, "won"),
            (GameStatus::Lost, "lost"),
        ];
        for (status, expected) in cases {
            assert_eq!(status.to_string(), expected, "for {status:?}");
        }
    }

    #[test]
    fn display_respects_width() {
        assert_eq!(format!("{:>5}", GameStatus::Won), "  won");
        assert_eq!(format!("{:<5}|", GameStatus::Won), "won  |");
    }
}
