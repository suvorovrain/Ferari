use minifb::{Key, Window};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// A snapshot of the input state at a specific moment in time.
///
/// This struct provides a view of all tracked key states.
pub struct InputSnapshot {
    /// Indicates if the W key (up movement) was pressed when the snapshot was taken
    pub up: bool,
    /// Indicates if the A key (left movement) was pressed when the snapshot was taken
    pub left: bool,
    /// Indicates if the S key (down movement) was pressed when the snapshot was taken
    pub down: bool,
    /// Indicates if the D key (right movement) was pressed when the snapshot was taken
    pub right: bool,
    /// Indicates if the Escape key was pressed when the snapshot was taken
    pub escape: bool,
}

/// Represents the current state of input keys.
///
/// This struct provides a way to track the state of specific keyboard keys (W, A, S, D, Escape).
#[derive(Clone)]
pub struct InputState {
    /// Tracks whether the W key (up movement) is currently pressed
    pub up: Arc<AtomicBool>,
    /// Tracks whether the A key (left movement) is currently pressed
    pub left: Arc<AtomicBool>,
    /// Tracks whether the S key (down movement) is currently pressed
    pub down: Arc<AtomicBool>,
    /// Tracks whether the D key (right movement) is currently pressed
    pub right: Arc<AtomicBool>,
    /// Tracks whether the Escape key is currently pressed
    pub escape: Arc<AtomicBool>,
}

impl InputState {
    /// Creates a new `InputState` with all keys initially not pressed.
    ///
    /// # Returns
    ///
    /// A new `InputState` instance with all values initialized to `false`.
    pub fn new() -> Self {
        Self {
            up: Arc::new(AtomicBool::new(false)),
            down: Arc::new(AtomicBool::new(false)),
            left: Arc::new(AtomicBool::new(false)),
            right: Arc::new(AtomicBool::new(false)),
            escape: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Updates the input state by querying the current key states from the window.
    ///
    /// This method checks the current state of the tracked keys (W, A, S, D, Escape)
    /// in the provided window and updates the internal values accordingly.
    ///
    /// # Parameters
    ///
    /// * `window` - A reference to the minifb `Window` to query for key states
    pub fn update(&self, window: &Window) {
        self.up.store(window.is_key_down(Key::W), Ordering::Relaxed);
        self.down.store(window.is_key_down(Key::S), Ordering::Relaxed);
        self.left.store(window.is_key_down(Key::A), Ordering::Relaxed);
        self.right.store(window.is_key_down(Key::D), Ordering::Relaxed);
        self.escape.store(window.is_key_down(Key::Escape), Ordering::Relaxed);
    }

    /// Reads the current state of all tracked keys and returns an `InputSnapshot`.
    ///
    /// This method creates a snapshot of the current input state by
    /// reading all key states.
    ///
    /// # Returns
    ///
    /// An `InputSnapshot` containing the current state of all tracked keys.
    pub fn read(&self) -> InputSnapshot {
        InputSnapshot {
            up: self.up.load(Ordering::Relaxed),
            down: self.down.load(Ordering::Relaxed),
            left: self.left.load(Ordering::Relaxed),
            right: self.right.load(Ordering::Relaxed),
            escape: self.escape.load(Ordering::Relaxed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that InputState initializes with all keys set to false
    #[test]
    fn test_new_input_state_initializes_all_false() {
        let input_state = InputState::new();
        let snapshot = input_state.read();

        assert!(!snapshot.up);
        assert!(!snapshot.down);
        assert!(!snapshot.left);
        assert!(!snapshot.right);
        assert!(!snapshot.escape);
    }

    /// Test that InputState can be cloned and both instances share state
    #[test]
    fn test_input_state_clone_shares_state() {
        let input_state1 = InputState::new();
        let input_state2 = input_state1.clone();

        // Both should start with the same state
        let snapshot1 = input_state1.read();
        let snapshot2 = input_state2.read();

        assert_eq!(snapshot1.up, snapshot2.up);
        assert_eq!(snapshot1.down, snapshot2.down);
        assert_eq!(snapshot1.left, snapshot2.left);
        assert_eq!(snapshot1.right, snapshot2.right);
        assert_eq!(snapshot1.escape, snapshot2.escape);
    }
}
