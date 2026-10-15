//! Telling the player about a newer release on GitHub. Only the GitHub release build does
//! this (the `update-check` feature); the Microsoft Store updates its apps itself.

mod github;
mod latest_release;
mod update_check;
mod version;

pub use github::open_release_page;
// Only the code that talks to GitHub reads its answer.
#[cfg(feature = "update-check")]
pub use latest_release::latest_version;
pub use latest_release::newer;
pub use update_check::UpdateCheck;
pub use version::Version;
