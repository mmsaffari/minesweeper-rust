use std::collections::BTreeSet;

use super::{Seen, View};
use crate::logic::Position;

/// What one opened number says: "exactly `mines` of these `unknown` cells are mines".
///
/// `unknown` is the number's hidden (or `?`) neighbours. Flagged neighbours are taken to be
/// mines and already subtracted from `mines`: the hint trusts the player's flags.
/// A `BTreeSet` (not a `HashSet`) keeps the cells sorted, so the solver always picks the
/// same cell first and its answers are predictable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clue {
    pub unknown: BTreeSet<Position>,
    pub mines: usize,
}

impl Clue {
    /// Every number on the board that still says something, i.e. has unknown neighbours.
    /// A number with more flags around it than its count can't be right (a flag is wrong),
    /// so it's left out instead of being trusted.
    pub fn all(view: &View) -> Vec<Clue> {
        view.cells()
            .filter_map(|(pos, seen)| match seen {
                Seen::Number(n) => Clue::of(view, pos, usize::from(n)),
                _ => None,
            })
            .collect()
    }

    fn of(view: &View, pos: Position, count: usize) -> Option<Clue> {
        let mut unknown = BTreeSet::new();
        let mut flags = 0;
        for neighbour in view.neighbours(pos) {
            match view.get(neighbour) {
                Some(Seen::Hidden | Seen::Questioned) => {
                    unknown.insert(neighbour);
                }
                Some(Seen::Flagged | Seen::Mine) => flags += 1,
                Some(Seen::Number(_)) | None => {}
            }
        }
        let mines = count.checked_sub(flags)?; // `None` if there are too many flags
        (!unknown.is_empty()).then_some(Clue { unknown, mines })
    }
}
