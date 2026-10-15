use std::fmt;
use std::io;

/// Why loading or saving a file failed.
///
/// Rust has no exceptions: a function that can fail returns `Result<T, SomeError>`.
/// This enum is "SomeError" for a saved file. It wraps the two things that can go wrong.
#[derive(Debug)]
pub enum StorageError {
    /// Reading or writing the file failed (no permission, disk full, ...).
    Io(io::Error),
    /// The file exists but isn't valid JSON for it (edited by hand, half written, ...).
    Json(serde_json::Error),
}

/// These two `From` impls are what make `?` work: inside a function returning
/// `Result<_, StorageError>`, `fs::read_to_string(path)?` turns an `io::Error`
/// into a `StorageError::Io` automatically before returning it.
impl From<io::Error> for StorageError {
    fn from(error: io::Error) -> Self {
        StorageError::Io(error)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(error: serde_json::Error) -> Self {
        StorageError::Json(error)
    }
}

/// The human-readable message, for `{}` / `.to_string()`. Like overriding `Exception.Message`.
impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::Io(error) => write!(f, "file error: {error}"),
            StorageError::Json(error) => write!(f, "not valid data: {error}"),
        }
    }
}

/// Marks it as a proper error type, so it works with the rest of Rust's error tooling.
impl std::error::Error for StorageError {}
