use serde::{Deserialize, Serialize};

/// The player's preferences, saved as `settings.json` next to the best times.
///
/// `#[serde(default)]`: a setting missing from the file (e.g. one added in a later version)
/// takes its value from `Default`, so old files keep working.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Look for a newer release on GitHub once at startup (only in builds that can).
    pub check_for_updates: bool,
}

/// Written by hand because the default for `bool` would be `false`, and checking for
/// updates is on unless the player turns it off.
impl Default for Settings {
    fn default() -> Self {
        Self {
            check_for_updates: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checking_for_updates_is_on_by_default() {
        assert!(Settings::default().check_for_updates);
    }

    #[test]
    fn an_empty_or_older_file_gets_the_defaults() {
        let settings: Settings = serde_json::from_str("{}").expect("valid");
        assert_eq!(settings, Settings::default());
    }

    #[test]
    fn a_turned_off_setting_stays_off() {
        let settings: Settings = serde_json::from_str(r#"{"check_for_updates": false}"#).expect("valid");
        assert!(!settings.check_for_updates);
    }
}
