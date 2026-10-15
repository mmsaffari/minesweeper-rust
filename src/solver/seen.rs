/// What the player can see of one cell. Unlike `logic::Cell`, there's no way to ask whether
/// a hidden cell is a mine: anything built on this (the export, the hint) can't cheat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seen {
    Hidden,
    Questioned,
    Flagged,
    /// An opened cell and its count (0 for an empty one).
    Number(u8),
    /// An opened mine: the game is lost.
    Mine,
}

impl Seen {
    /// The character used in the text form of a board, the same as in the game's tests:
    /// `#` hidden, `?` question mark, `F` flag, `.` empty, `1`–`8` numbers, `*` opened mine.
    pub fn to_char(self) -> char {
        match self {
            Seen::Hidden => '#',
            Seen::Questioned => '?',
            Seen::Flagged => 'F',
            Seen::Number(0) => '.',
            Seen::Number(n) => char::from(b'0' + n),
            Seen::Mine => '*',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_of_cell_has_its_character() {
        let cases = [
            (Seen::Hidden, '#'),
            (Seen::Questioned, '?'),
            (Seen::Flagged, 'F'),
            (Seen::Number(0), '.'),
            (Seen::Number(1), '1'),
            (Seen::Number(8), '8'),
            (Seen::Mine, '*'),
        ];
        for (seen, expected) in cases {
            assert_eq!(seen.to_char(), expected, "{seen:?}");
        }
    }
}
