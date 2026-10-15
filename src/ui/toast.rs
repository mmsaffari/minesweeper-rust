/// A short message shown over the board for a moment ("Board copied", ...), then gone.
///
/// The time it disappears is only fixed when it's first shown (`start`), because the code
/// that creates a toast (`App::apply`) must not read the clock: that keeps `apply` testable.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    text: String,
    /// Seconds since the program started (macroquad's `get_time()`) when it disappears.
    until: Option<f64>,
}

/// How long a toast stays, in seconds.
const DURATION: f64 = 2.5;

impl Toast {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            until: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Starts the countdown on the first frame it's shown; later calls change nothing.
    pub fn start(&mut self, now: f64) {
        self.until.get_or_insert(now + DURATION);
    }

    pub fn is_visible(&self, now: f64) -> bool {
        self.until.is_some_and(|until| now < until)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_visible_from_its_start_until_the_duration_is_over() {
        let mut toast = Toast::new("hello");
        assert!(!toast.is_visible(10.0), "not started yet");
        toast.start(10.0);
        assert!(toast.is_visible(10.0));
        assert!(toast.is_visible(10.0 + DURATION - 0.1));
        assert!(!toast.is_visible(10.0 + DURATION));
    }

    #[test]
    fn starting_again_does_not_restart_the_countdown() {
        let mut toast = Toast::new("hello");
        toast.start(10.0);
        toast.start(11.0);
        assert!(!toast.is_visible(10.0 + DURATION));
    }
}
