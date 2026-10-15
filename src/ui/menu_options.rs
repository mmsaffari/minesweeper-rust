/// The menu items that are only there sometimes, all in the Help menu.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MenuOptions {
    /// "Check for updates": only in a build that can check (the GitHub release build).
    pub update_toggle: bool,
    /// "Get new version...": only once a newer release has been found.
    pub update_available: bool,
}
