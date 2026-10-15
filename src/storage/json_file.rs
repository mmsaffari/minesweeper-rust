use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::StorageError;

/// Where a file called `name` belongs in this game's data folder, or `None` if the system
/// has no home folder. Linux: `~/.local/share/minesweeper-rust/<name>`;
/// Windows: `%APPDATA%\minesweeper-rust\data\<name>`.
pub fn data_file(name: &str) -> Option<PathBuf> {
    ProjectDirs::from("", "", "minesweeper-rust").map(|dirs| dirs.data_dir().join(name))
}

/// Reads any `T` that serde can turn JSON into. A missing file is not an error: nothing has
/// been saved yet, so it's `T::default()`.
///
/// The bounds say what `T` must be able to do: `DeserializeOwned` (built from JSON, owning
/// its data rather than borrowing from the text) and `Default` (for the missing file).
/// In C# this would be `T Load<T>(string path) where T : new()`, plus a JSON attribute.
pub fn load<T: DeserializeOwned + Default>(path: &Path) -> Result<T, StorageError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(T::default()),
        Err(error) => return Err(error.into()),
    };
    // `?`: if parsing fails, return the error right here (converted by `From`), else unwrap.
    Ok(serde_json::from_str(&text)?)
}

/// Writes `value` as JSON, creating the folder first if it doesn't exist yet.
pub fn save<T: Serialize>(path: &Path, value: &T) -> Result<(), StorageError> {
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder)?;
    }
    fs::write(path, serde_json::to_string_pretty(value)?)?;
    Ok(())
}

/// For startup: loads the file, but a broken file must not stop the game from starting.
/// It's reported and the default is used (the next save overwrites the broken file).
pub fn load_or_default<T: DeserializeOwned + Default>(path: &Path) -> T {
    load(path).unwrap_or_else(|error| {
        eprintln!("Couldn't read {}: {error}", path.display());
        T::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    /// A small stand-in for the real files (best times, settings): any serde type works.
    #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(default)]
    struct Example {
        name: String,
        count: u32,
    }

    /// A file path of our own in the system's temp folder, so tests never touch real files
    /// and never collide with each other (each test passes a different name).
    fn temp_path(name: &str) -> PathBuf {
        let folder = std::env::temp_dir().join(format!("minesweeper-tests-{}", std::process::id()));
        folder.join(name).join("example.json")
    }

    #[test]
    fn what_is_saved_loads_back_the_same() {
        let path = temp_path("round_trip");
        let value = Example {
            name: "mine".to_owned(),
            count: 42,
        };
        save(&path, &value).expect("save");
        assert_eq!(load::<Example>(&path).expect("load"), value);
    }

    #[test]
    fn save_creates_missing_folders() {
        let path = temp_path("deep").join("er").join("example.json");
        save(&path, &Example::default()).expect("save into folders that don't exist yet");
        assert!(path.exists());
    }

    #[test]
    fn a_missing_file_gives_the_default() {
        let path = temp_path("never_written");
        assert_eq!(load::<Example>(&path).expect("missing is fine"), Example::default());
    }

    #[test]
    fn a_broken_file_is_a_json_error_but_startup_still_works() {
        let path = temp_path("broken");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "this is not json").unwrap();

        assert!(matches!(load::<Example>(&path), Err(StorageError::Json(_))));
        assert_eq!(load_or_default::<Example>(&path), Example::default());
    }

    #[test]
    fn a_file_missing_some_fields_still_loads() {
        let path = temp_path("partial");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, r#"{"count": 7}"#).unwrap();
        assert_eq!(
            load::<Example>(&path).expect("missing fields fall back to their defaults"),
            Example {
                name: String::new(),
                count: 7
            }
        );
    }
}
