//! Everything that talks to GitHub. Only compiled in with the `update-check` feature (the
//! GitHub release build); without it, these do nothing, and the HTTP library isn't even
//! part of the program.

use super::Version;

/// `https://github.com/mmsaffari/minesweeper-rust`, from Cargo.toml.
#[cfg(feature = "update-check")]
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// Asks GitHub for the latest release. Blocks until GitHub answers (at most 10 seconds),
/// so it must run on its own thread, never in the frame loop. Any failure (offline, GitHub
/// down, rate limit) is simply `None`: nothing to report to the player.
#[cfg(feature = "update-check")]
pub fn fetch_latest_version() -> Option<Version> {
    use std::time::Duration;

    let api = REPOSITORY.replacen("https://github.com/", "https://api.github.com/repos/", 1);
    let url = format!("{api}/releases/latest");

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .into();
    let mut response = agent
        .get(&url)
        // GitHub's API rejects requests without a User-Agent.
        .header("User-Agent", concat!("minesweeper-rust/", env!("CARGO_PKG_VERSION")))
        .header("Accept", "application/vnd.github+json")
        .call()
        .ok()?;
    let json = response.body_mut().read_to_string().ok()?;
    super::latest_version(&json)
}

#[cfg(not(feature = "update-check"))]
pub fn fetch_latest_version() -> Option<Version> {
    None
}

/// Opens the latest release's page in the player's web browser.
#[cfg(feature = "update-check")]
pub fn open_release_page() {
    if let Err(error) = open::that(format!("{REPOSITORY}/releases/latest")) {
        eprintln!("Couldn't open the browser: {error}");
    }
}

#[cfg(not(feature = "update-check"))]
pub fn open_release_page() {}

#[cfg(all(test, feature = "update-check"))]
mod tests {
    use super::*;

    /// Goes online, so it's `#[ignore]`d: run it by hand with
    /// `cargo test --features update-check -- --ignored`.
    #[test]
    #[ignore = "needs the internet"]
    fn github_answers_with_a_release_version() {
        let latest = fetch_latest_version().expect("GitHub's latest release");
        assert!(latest >= Version::parse("1.3.2").unwrap(), "{latest}");
    }
}
