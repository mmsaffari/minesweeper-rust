#[cfg(any(feature = "update-check", test))]
use serde::Deserialize;

use super::Version;

/// The one field we need from GitHub's "latest release" answer. The answer has dozens of
/// fields; serde simply ignores the ones a struct doesn't mention.
#[cfg(any(feature = "update-check", test))]
#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

/// The version of the latest release, read from the JSON of GitHub's API
/// (`GET /repos/{owner}/{repo}/releases/latest`). `None` if the answer isn't what we expect.
#[cfg(any(feature = "update-check", test))]
pub fn latest_version(json: &str) -> Option<Version> {
    let release: Release = serde_json::from_str(json).ok()?;
    Version::parse(&release.tag_name)
}

/// Whether `latest` is worth telling the player about: only when it's newer than `current`.
pub fn newer(latest: Version, current: Version) -> Option<Version> {
    (latest > current).then_some(latest)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(major: u32, minor: u32, patch: u32) -> Version {
        Version { major, minor, patch }
    }

    #[test]
    fn reads_the_tag_and_ignores_everything_else() {
        let json = r#"{"url": "https://...", "tag_name": "v1.4.0", "name": "Minesweeper v1.4.0",
                       "assets": [{"name": "minesweeper-v1.4.0-windows.zip"}], "draft": false}"#;
        assert_eq!(latest_version(json), Some(v(1, 4, 0)));
    }

    #[test]
    fn an_unexpected_answer_gives_nothing() {
        for json in [
            "",
            "not json",
            "{}",
            r#"{"tag_name": "nightly"}"#,
            r#"{"message": "API rate limit exceeded"}"#,
        ] {
            assert_eq!(latest_version(json), None, "{json:?}");
        }
    }

    #[test]
    fn only_a_newer_version_counts() {
        let current = v(1, 3, 2);
        assert_eq!(newer(v(1, 4, 0), current), Some(v(1, 4, 0)));
        assert_eq!(newer(v(1, 3, 2), current), None, "the same version");
        assert_eq!(
            newer(v(1, 3, 1), current),
            None,
            "an older one (e.g. a newer release was deleted)"
        );
    }
}
