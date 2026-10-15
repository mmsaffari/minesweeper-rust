use macroquad::input::{KeyCode, is_key_down, is_key_pressed};

use super::Command;
use crate::logic::Difficulty;

/// Every key that does something. `keyboard_command` only asks macroquad about these.
const SHORTCUT_KEYS: [KeyCode; 9] = [
    KeyCode::F1,
    KeyCode::F2,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Kp1,
    KeyCode::Kp2,
    KeyCode::Kp3,
    KeyCode::C,
];

/// The shortcut pressed this frame, if any.
pub fn keyboard_command() -> Option<Command> {
    let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
    SHORTCUT_KEYS
        .into_iter()
        .filter(|&key| is_key_pressed(key))
        .find_map(|key| command_for_key(key, ctrl))
}

/// F1 = About, F2 = new game (both as in the original), 1/2/3 = Beginner/Intermediate/Expert
/// (the number pad works too), Ctrl+C = copy the board as text.
/// `ctrl`: a Control key is held down. Plain keys only count without it, so Ctrl+1 does nothing.
pub fn command_for_key(key: KeyCode, ctrl: bool) -> Option<Command> {
    match (key, ctrl) {
        (KeyCode::C, true) => Some(Command::CopyBoard),
        (_, true) => None,
        (KeyCode::F1, false) => Some(Command::About),
        (KeyCode::F2, false) => Some(Command::NewGame),
        (KeyCode::Key1 | KeyCode::Kp1, false) => Some(Command::Start(Difficulty::Beginner)),
        (KeyCode::Key2 | KeyCode::Kp2, false) => Some(Command::Start(Difficulty::Intermediate)),
        (KeyCode::Key3 | KeyCode::Kp3, false) => Some(Command::Start(Difficulty::Expert)),
        _ => None, // KeyCode has ~120 variants; every other key does nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcuts_map_to_their_commands() {
        let cases = [
            (KeyCode::F1, false, Some(Command::About)),
            (KeyCode::F2, false, Some(Command::NewGame)),
            (KeyCode::Key1, false, Some(Command::Start(Difficulty::Beginner))),
            (KeyCode::Kp1, false, Some(Command::Start(Difficulty::Beginner))),
            (KeyCode::Key2, false, Some(Command::Start(Difficulty::Intermediate))),
            (KeyCode::Kp2, false, Some(Command::Start(Difficulty::Intermediate))),
            (KeyCode::Key3, false, Some(Command::Start(Difficulty::Expert))),
            (KeyCode::Kp3, false, Some(Command::Start(Difficulty::Expert))),
            (KeyCode::C, true, Some(Command::CopyBoard)),
            (KeyCode::A, false, None),
            (KeyCode::Key4, false, None),
            (KeyCode::Escape, false, None),
        ];
        for (key, ctrl, expected) in cases {
            assert_eq!(command_for_key(key, ctrl), expected, "{key:?}, ctrl: {ctrl}");
        }
    }

    #[test]
    fn control_changes_what_a_key_means() {
        assert_eq!(command_for_key(KeyCode::C, false), None, "plain C does nothing");
        assert_eq!(command_for_key(KeyCode::Key1, true), None, "Ctrl+1 is not Beginner");
        assert_eq!(command_for_key(KeyCode::F2, true), None);
    }

    /// Guards against adding a key to the mapping but forgetting it in `SHORTCUT_KEYS`
    /// (or the other way round), which would make it silently do nothing.
    #[test]
    fn every_listed_shortcut_key_has_a_command() {
        for key in SHORTCUT_KEYS {
            let with_or_without_ctrl = command_for_key(key, false).or(command_for_key(key, true));
            assert!(with_or_without_ctrl.is_some(), "{key:?} is listed but does nothing");
        }
    }
}
