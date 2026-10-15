//! Saving and loading the game's small JSON files (best times, settings) in the user's
//! data folder. Generic: any type serde can (de)serialize works.

mod json_file;
mod storage_error;
mod stored;

pub use storage_error::StorageError;
pub use stored::Stored;

/// For tests elsewhere in the crate: a Stored value that never touches a real file.
#[cfg(test)]
pub(crate) use stored::tests::in_memory;
