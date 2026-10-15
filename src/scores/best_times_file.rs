use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use super::{BestTimes, BestTimesError};

/// Where the best times live on this system, or `None` if the system has no home folder.
/// Linux: `~/.local/share/minesweeper-rust/best_times.json`
/// Windows: `%APPDATA%\minesweeper-rust\data\best_times.json`
pub fn default_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "minesweeper-rust").map(|dirs| dirs.data_dir().join("best_times.json"))
}

/// Reads the best times. A missing file is not an error: nobody has won yet, so it's empty.
pub fn load(path: &Path) -> Result<BestTimes, BestTimesError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(BestTimes::default()),
        Err(error) => return Err(error.into()),
    };
    // `?`: if parsing fails, return the error right here (converted by `From`), else unwrap.
    let best = serde_json::from_str(&text)?;
    Ok(best)
}

/// Writes the best times, creating the folder first if it doesn't exist yet.
pub fn save(path: &Path, best: &BestTimes) -> Result<(), BestTimesError> {
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder)?;
    }
    fs::write(path, serde_json::to_string_pretty(best)?)?;
    Ok(())
}

/// For startup: loads the file, but a broken file must not stop the game from starting.
/// It's reported and the game starts with no records (the next record overwrites the file).
pub fn load_or_default(path: &Path) -> BestTimes {
    load(path).unwrap_or_else(|error| {
        eprintln!("Couldn't read best times from {}: {error}", path.display());
        BestTimes::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::Difficulty;

    /// A file path of our own in the system's temp folder, so tests never touch the real
    /// records and never collide with each other (each test passes a different name).
    fn temp_path(name: &str) -> PathBuf {
        let folder = std::env::temp_dir().join(format!("minesweeper-tests-{}", std::process::id()));
        folder.join(name).join("best_times.json")
    }

    #[test]
    fn what_is_saved_loads_back_the_same() {
        let path = temp_path("round_trip");
        let mut best = BestTimes::default();
        best.record(Difficulty::Beginner, 42);
        best.record(Difficulty::Expert, 321);

        save(&path, &best).expect("save");
        assert_eq!(load(&path).expect("load"), best);
    }

    #[test]
    fn save_creates_missing_folders() {
        let path = temp_path("deep").join("er").join("best_times.json");
        save(&path, &BestTimes::default()).expect("save into folders that don't exist yet");
        assert!(path.exists());
    }

    #[test]
    fn a_missing_file_means_no_records_yet() {
        let path = temp_path("never_written");
        assert_eq!(load(&path).expect("missing is fine"), BestTimes::default());
    }

    #[test]
    fn a_broken_file_is_a_json_error() {
        let path = temp_path("broken");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "this is not json").unwrap();

        assert!(matches!(load(&path), Err(BestTimesError::Json(_))));
        assert_eq!(load_or_default(&path), BestTimes::default(), "startup still works");
    }

    #[test]
    fn a_file_missing_some_fields_still_loads() {
        let path = temp_path("partial");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, r#"{"beginner": 7}"#).unwrap();

        let best = load(&path).expect("missing fields fall back to None");
        assert_eq!(best.get(Difficulty::Beginner), Some(7));
        assert_eq!(best.get(Difficulty::Expert), None);
    }
}
