use std::fmt;

/// A release version like `1.3.2`.
///
/// The fields are in order of importance, so the derived `Ord` compares them exactly the
/// way versions compare: major first, then minor, then patch (`1.10.0` > `1.9.9`, which
/// comparing the *text* would get wrong).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// This program's own version, from Cargo.toml.
    pub fn current() -> Self {
        Self::parse(env!("CARGO_PKG_VERSION")).expect("Cargo versions are major.minor.patch")
    }

    /// Reads `1.3.2` or a release tag like `v1.3.2`. Anything else (a pre-release like
    /// `v2.0.0-beta`, or not a version at all) gives `None`.
    pub fn parse(text: &str) -> Option<Self> {
        let mut parts = text.strip_prefix('v').unwrap_or(text).split('.');
        let mut next = || parts.next()?.parse::<u32>().ok();
        let version = Self {
            major: next()?,
            minor: next()?,
            patch: next()?,
        };
        // Exactly three parts: "1.2.3.4" isn't a version we publish.
        parts.next().is_none().then_some(version)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(major: u32, minor: u32, patch: u32) -> Version {
        Version { major, minor, patch }
    }

    #[test]
    fn reads_versions_and_release_tags() {
        assert_eq!(Version::parse("1.3.2"), Some(v(1, 3, 2)));
        assert_eq!(Version::parse("v1.3.2"), Some(v(1, 3, 2)));
        assert_eq!(Version::parse("v10.0.15"), Some(v(10, 0, 15)));
    }

    #[test]
    fn rejects_anything_else() {
        for text in ["", "v", "1.3", "1.3.2.1", "v2.0.0-beta", "latest", "1.x.0", "-1.0.0"] {
            assert_eq!(Version::parse(text), None, "{text:?}");
        }
    }

    #[test]
    fn compares_by_number_not_by_text() {
        assert!(v(1, 10, 0) > v(1, 9, 9), "as text, \"1.10.0\" < \"1.9.9\"");
        assert!(v(2, 0, 0) > v(1, 99, 99));
        assert!(v(1, 3, 2) > v(1, 3, 1));
    }

    #[test]
    fn shows_without_the_v() {
        assert_eq!(v(1, 4, 0).to_string(), "1.4.0");
    }

    #[test]
    fn its_own_version_is_readable() {
        assert_eq!(Version::current().to_string(), env!("CARGO_PKG_VERSION"));
    }
}
