use std::path::PathBuf;

use serde::Serialize;
use serde::de::DeserializeOwned;

use super::json_file::{data_file, load_or_default, save};

/// A value together with the file it's kept in (best times, settings, ...).
///
/// It's generic: `Stored<BestTimes>` and `Stored<Settings>` are the same code for two types.
/// The `impl` block's bounds say which types qualify: ones serde can write and read as
/// JSON, and that have a `Default` for when nothing has been saved yet.
#[derive(Debug)]
pub struct Stored<T> {
    pub value: T,
    /// `None` keeps it in memory only: tests use that, so they never touch real files.
    path: Option<PathBuf>,
}

impl<T: Serialize + DeserializeOwned + Default> Stored<T> {
    /// Loads `file_name` from the game's data folder (or the default, if there's none yet).
    pub fn load(file_name: &str) -> Self {
        let path = data_file(file_name);
        let value = path.as_deref().map(load_or_default).unwrap_or_default();
        Self { value, path }
    }

    /// Writes the value to its file. Failing to save (no permission, disk full) is reported
    /// but must not stop the game.
    pub fn save(&self) {
        if let Some(path) = &self.path
            && let Err(error) = save(path, &self.value)
        {
            eprintln!("Couldn't save {}: {error}", path.display());
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A value that is never written anywhere, for tests elsewhere in the crate (`App`'s).
    pub(crate) fn in_memory<T>(value: T) -> Stored<T> {
        Stored { value, path: None }
    }

    #[test]
    fn saving_an_in_memory_value_writes_nothing_and_does_not_fail() {
        let stored = in_memory(crate::config::Settings::default());
        stored.save(); // no path: nothing happens, no panic
        assert!(stored.value.check_for_updates);
    }
}
