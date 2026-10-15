use std::fmt;
use std::io;

/// Why loading or saving the best times failed.
///
/// Rust has no exceptions: a function that can fail returns `Result<T, SomeError>`.
/// This enum is "SomeError" for the best-times file. It wraps the two things that can go wrong.
#[derive(Debug)]
pub enum BestTimesError {
    /// Reading or writing the file failed (no permission, disk full, ...).
    Io(io::Error),
    /// The file exists but isn't valid best-times JSON (edited by hand, half written, ...).
    Json(serde_json::Error),
}

/// These two `From` impls are what make `?` work: inside a function returning
/// `Result<_, BestTimesError>`, `fs::read_to_string(path)?` turns an `io::Error`
/// into a `BestTimesError::Io` automatically before returning it.
impl From<io::Error> for BestTimesError {
    fn from(error: io::Error) -> Self {
        BestTimesError::Io(error)
    }
}

impl From<serde_json::Error> for BestTimesError {
    fn from(error: serde_json::Error) -> Self {
        BestTimesError::Json(error)
    }
}

/// The human-readable message, for `{}` / `.to_string()`. Like overriding `Exception.Message`.
impl fmt::Display for BestTimesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BestTimesError::Io(error) => write!(f, "file error: {error}"),
            BestTimesError::Json(error) => write!(f, "not valid best-times data: {error}"),
        }
    }
}

/// Marks it as a proper error type, so it works with the rest of Rust's error tooling.
impl std::error::Error for BestTimesError {}
