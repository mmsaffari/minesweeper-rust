use std::thread;
use std::time::{Duration, Instant};

/// Keeps the game at a steady frame rate by sleeping between frames.
///
/// Without it, the game draws as fast as the graphics driver lets it: one frame per screen
/// refresh at best (144 or more on fast screens), and some drivers keep a CPU core busy while
/// they wait for the next refresh. Minesweeper needs neither: 60 frames a second is plenty, and
/// while the game sleeps, the CPU rests.
#[derive(Debug)]
pub struct FramePacer {
    frame: Duration,
    /// When the next frame should start.
    next: Instant,
}

impl FramePacer {
    pub fn new(frames_per_second: u32, now: Instant) -> Self {
        let frame = Duration::from_secs(1) / frames_per_second;
        Self {
            frame,
            next: now + frame,
        }
    }

    /// Sleeps until it's time for the next frame.
    pub fn wait(&mut self) {
        thread::sleep(self.delay(Instant::now()));
    }

    /// How long to sleep at `now`, and moves on to the frame after. Kept apart from `wait`,
    /// which reads the real clock, so tests can pass any time.
    fn delay(&mut self, now: Instant) -> Duration {
        let delay = self.next.saturating_duration_since(now);
        // Counting from the planned start, not from `now`, keeps the rate steady even though
        // each frame's work takes a different time.
        self.next += self.frame;
        // A frame that took far too long (the window being dragged, say): start counting
        // afresh instead of racing through the missed frames to catch up.
        if self.next < now {
            self.next = now + self.frame;
        }
        delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME: Duration = Duration::from_millis(10); // 100 frames a second

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    #[test]
    fn sleeps_for_what_is_left_of_the_frame() {
        let start = Instant::now();
        let mut pacer = FramePacer::new(100, start);
        assert_eq!(pacer.frame, FRAME);
        assert_eq!(pacer.delay(start + ms(3)), ms(7), "3 ms of work, 7 ms of rest");
        assert_eq!(pacer.delay(start + ms(18)), ms(2), "the second frame ends at 20 ms");
    }

    #[test]
    fn a_slow_frame_is_not_followed_by_a_burst_of_fast_ones() {
        let start = Instant::now();
        let mut pacer = FramePacer::new(100, start);
        assert_eq!(pacer.delay(start + ms(55)), Duration::ZERO, "late: no sleep");
        // The schedule restarts from 55 ms instead of trying to fit in the missed frames.
        assert_eq!(pacer.delay(start + ms(56)), ms(9));
    }
}
