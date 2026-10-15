use std::sync::mpsc::{self, Receiver};
use std::thread;

use super::{Version, github, newer};

/// A check for a newer release, running in the background.
///
/// The request to GitHub can take seconds (or time out), and the frame loop must never wait
/// that long, so it runs on its own thread. The thread sends its one result through a
/// *channel*: the sending end moves into the thread, the receiving end stays here, and
/// `poll` looks at it once per frame without waiting. (In C#, this would be a `Task` whose
/// result the game loop checks with `IsCompleted`.)
#[derive(Debug, Default)]
pub struct UpdateCheck {
    /// `None` when no check is running (never started, or already answered).
    result: Option<Receiver<Option<Version>>>,
}

impl UpdateCheck {
    /// Starts a check, if this build can do one (the `update-check` feature).
    pub fn start() -> Self {
        if !Self::is_supported() {
            return Self::default();
        }
        let (sender, receiver) = mpsc::channel();
        // `move`: the closure takes ownership of `sender`, so it can outlive this function.
        thread::spawn(move || {
            let update = github::fetch_latest_version().and_then(|latest| newer(latest, Version::current()));
            // If the game has quit in the meantime, nobody is listening: that's fine.
            let _ = sender.send(update);
        });
        Self { result: Some(receiver) }
    }

    /// Whether this build can check for updates at all.
    pub fn is_supported() -> bool {
        cfg!(feature = "update-check")
    }

    /// The newer version, on the one frame its answer arrives; otherwise `None`.
    pub fn poll(&mut self) -> Option<Version> {
        let answer = self.result.as_ref()?.try_recv().ok()?;
        self.result = None; // one answer only: stop listening
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_check_that_never_started_never_answers() {
        let mut check = UpdateCheck::default();
        assert_eq!(check.poll(), None);
    }

    /// The channel part, without the network: a thread sends one answer, `poll` picks it
    /// up exactly once.
    #[test]
    fn an_answer_is_picked_up_once() {
        let (sender, receiver) = mpsc::channel();
        let mut check = UpdateCheck { result: Some(receiver) };
        let newer = Version {
            major: 99,
            minor: 0,
            patch: 0,
        };
        thread::spawn(move || sender.send(Some(newer)).unwrap()).join().unwrap();

        assert_eq!(check.poll(), Some(newer));
        assert_eq!(check.poll(), None, "already answered");
    }

    #[test]
    fn polling_before_the_answer_arrives_does_not_wait() {
        let (_sender, receiver) = mpsc::channel();
        let mut check = UpdateCheck { result: Some(receiver) };
        assert_eq!(check.poll(), None);
        assert!(check.result.is_some(), "still listening");
    }
}
