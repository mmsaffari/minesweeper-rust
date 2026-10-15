use macroquad::input::{KeyCode, is_key_pressed};

use super::Command;
use crate::logic::Difficulty;

/// Every key that does something. `keyboard_command` only asks macroquad about these.
const SHORTCUT_KEYS: [KeyCode; 7] = [
    KeyCode::F2,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Kp1,
    KeyCode::Kp2,
    KeyCode::Kp3,
];

/// The shortcut pressed this frame, if any.
pub fn keyboard_command() -> Option<Command> {
    SHORTCUT_KEYS
        .into_iter()
        .filter(|&key| is_key_pressed(key))
        .find_map(command_for_key)
}

/// F2 = new game (as in the original), 1/2/3 = Beginner/Intermediate/Expert.
/// The number pad works too.
pub fn command_for_key(key: KeyCode) -> Option<Command> {
    match key {
        KeyCode::F2 => Some(Command::NewGame),
        KeyCode::Key1 | KeyCode::Kp1 => Some(Command::Start(Difficulty::Beginner)),
        KeyCode::Key2 | KeyCode::Kp2 => Some(Command::Start(Difficulty::Intermediate)),
        KeyCode::Key3 | KeyCode::Kp3 => Some(Command::Start(Difficulty::Expert)),
        _ => None, // KeyCode has ~120 variants; every other key does nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcuts_map_to_their_commands() {
        let cases = [
            (KeyCode::F2, Some(Command::NewGame)),
            (KeyCode::Key1, Some(Command::Start(Difficulty::Beginner))),
            (KeyCode::Kp1, Some(Command::Start(Difficulty::Beginner))),
            (KeyCode::Key2, Some(Command::Start(Difficulty::Intermediate))),
            (KeyCode::Kp2, Some(Command::Start(Difficulty::Intermediate))),
            (KeyCode::Key3, Some(Command::Start(Difficulty::Expert))),
            (KeyCode::Kp3, Some(Command::Start(Difficulty::Expert))),
            (KeyCode::A, None),
            (KeyCode::Key4, None),
            (KeyCode::Escape, None),
        ];
        for (key, expected) in cases {
            assert_eq!(command_for_key(key), expected, "{key:?}");
        }
    }

    /// Guards against adding a key to the mapping but forgetting it in `SHORTCUT_KEYS`
    /// (or the other way round), which would make it silently do nothing.
    #[test]
    fn every_listed_shortcut_key_has_a_command() {
        for key in SHORTCUT_KEYS {
            assert!(command_for_key(key).is_some(), "{key:?} is listed but does nothing");
        }
    }
}
