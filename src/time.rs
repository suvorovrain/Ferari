use std::time::Instant;

/// Time tracking structure for game loops and frame timing.
///
/// Tracks delta time (time between frames) and total elapsed time.
pub struct Time {
    /// Interval between frames (sec)
    pub delta: f32,
    /// Total time elapsed (sec)
    pub total: f32,
    /// Delta calucalution
    last_instant: Instant,
    /// FPS print
    fps_timer: f32,
}

impl Time {
    /// Creates a new Time instance with zero values and starts tracking.
    ///
    /// # Returns
    ///
    /// A new `Time` instance with zero values of delta and total
    // and last_instant set to the current time.
    pub fn new() -> Self {
        Self { delta: 0.0, total: 0.0, last_instant: Instant::now(), fps_timer: 0.0 }
    }

    /// Updates time measurements.
    ///
    /// Calculates the time elapsed since the last update and updates
    /// both delta and total time values.
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_instant).as_secs_f32();
        self.total += self.delta;
        self.last_instant = now;
        self.fps_timer += self.delta;

        if self.fps_timer >= 1.0 {
            let fps = 1.0 / self.delta.max(1e-6);
            println!("FPS: {:.1}", fps);
            self.fps_timer = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    /// Test that Time initializes with correct default values
    #[test]
    fn test_initialization() {
        let time = Time::new();
        assert_eq!(time.delta, 0.0);
        assert_eq!(time.total, 0.0);
    }

    /// Test that update() method properly calculates delta time and updates total time
    #[test]
    fn test_update_updates_values() {
        let mut time = Time::new();
        let initial_total = time.total;

        thread::sleep(Duration::from_millis(10));
        time.update();

        assert!(time.delta > 0.0, "Delta should be positive after update");
        assert!(time.total > initial_total, "Total time should increase after update");
        assert!(time.total >= time.delta, "Total should be at least as large as delta");
    }

    /// Test that multiple consecutive updates work correctly
    #[test]
    fn test_multiple_updates() {
        let mut time = Time::new();

        time.update();
        let first_total = time.total;

        thread::sleep(Duration::from_millis(5));
        time.update();

        assert!(time.delta > 0.0, "Second delta should be positive");
        assert!(time.total > first_total, "Total should increase with each update");
        assert!(time.total == first_total + time.delta, "Total should be sum of all deltas");
    }

    /// Test that total time accurately accumulates delta values over multiple updates
    #[test]
    fn test_total_accumulation() {
        let mut time = Time::new();
        let mut expected_total = 0.0;

        for i in 0..3 {
            let sleep_duration = Duration::from_millis(10 * (i + 1));
            thread::sleep(sleep_duration);
            time.update();
            expected_total += time.delta;

            assert!(
                (time.total - expected_total).abs() < f32::EPSILON,
                "Total should accurately accumulate delta values"
            );
        }
    }
}
