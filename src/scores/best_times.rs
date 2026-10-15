use serde::{Deserialize, Serialize};

use crate::logic::Difficulty;

/// The fastest win per difficulty, in seconds. `None` until that difficulty has been won.
///
/// `Serialize`/`Deserialize` let serde turn this into JSON and back:
/// `{"beginner":42,"intermediate":null,"expert":null}`.
/// `#[serde(default)]` means a file with a missing field still loads (that field is `None`),
/// so adding a field later won't break existing files.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct BestTimes {
    beginner: Option<u64>,
    intermediate: Option<u64>,
    expert: Option<u64>,
}

impl BestTimes {
    pub fn get(&self, difficulty: Difficulty) -> Option<u64> {
        *self.slot(difficulty)
    }

    /// Remembers `seconds` if it beats the current best (or there is none yet).
    /// Returns whether it was a new record, so the caller knows whether to save.
    pub fn record(&mut self, difficulty: Difficulty, seconds: u64) -> bool {
        let slot = self.slot_mut(difficulty);
        let is_record = slot.is_none_or(|best| seconds < best);
        if is_record {
            *slot = Some(seconds);
        }
        is_record
    }

    fn slot(&self, difficulty: Difficulty) -> &Option<u64> {
        match difficulty {
            Difficulty::Beginner => &self.beginner,
            Difficulty::Intermediate => &self.intermediate,
            Difficulty::Expert => &self.expert,
        }
    }

    /// Hands out a mutable reference to one field, so `record` can change it in place.
    fn slot_mut(&mut self, difficulty: Difficulty) -> &mut Option<u64> {
        match difficulty {
            Difficulty::Beginner => &mut self.beginner,
            Difficulty::Intermediate => &mut self.intermediate,
            Difficulty::Expert => &mut self.expert,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Difficulty; 3] = [Difficulty::Beginner, Difficulty::Intermediate, Difficulty::Expert];

    #[test]
    fn starts_empty() {
        let best = BestTimes::default();
        for difficulty in ALL {
            assert_eq!(best.get(difficulty), None, "{difficulty:?}");
        }
    }

    #[test]
    fn the_first_win_is_always_a_record() {
        let mut best = BestTimes::default();
        assert!(best.record(Difficulty::Beginner, 120));
        assert_eq!(best.get(Difficulty::Beginner), Some(120));
    }

    #[test]
    fn only_a_faster_time_replaces_the_record() {
        let mut best = BestTimes::default();
        best.record(Difficulty::Expert, 300);

        assert!(!best.record(Difficulty::Expert, 400), "slower");
        assert!(!best.record(Difficulty::Expert, 300), "equal is not a new record");
        assert_eq!(best.get(Difficulty::Expert), Some(300));

        assert!(best.record(Difficulty::Expert, 250), "faster");
        assert_eq!(best.get(Difficulty::Expert), Some(250));
    }

    #[test]
    fn each_difficulty_keeps_its_own_record() {
        let mut best = BestTimes::default();
        best.record(Difficulty::Beginner, 10);
        best.record(Difficulty::Expert, 500);
        assert_eq!(best.get(Difficulty::Beginner), Some(10));
        assert_eq!(best.get(Difficulty::Intermediate), None);
        assert_eq!(best.get(Difficulty::Expert), Some(500));
    }
}
