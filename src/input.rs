use minifb::{Key, Window};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct InputState {
    pub up: Arc<AtomicBool>,
    pub down: Arc<AtomicBool>,
    pub left: Arc<AtomicBool>,
    pub right: Arc<AtomicBool>,
    pub escape: Arc<AtomicBool>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            up: Arc::new(AtomicBool::new(false)),
            down: Arc::new(AtomicBool::new(false)),
            left: Arc::new(AtomicBool::new(false)),
            right: Arc::new(AtomicBool::new(false)),
            escape: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn update(&self, window: &Window) {
        self.up.store(window.is_key_down(Key::W), Ordering::Relaxed);
        self.down.store(window.is_key_down(Key::S), Ordering::Relaxed);
        self.left.store(window.is_key_down(Key::A), Ordering::Relaxed);
        self.right.store(window.is_key_down(Key::D), Ordering::Relaxed);
        self.escape.store(window.is_key_down(Key::Escape), Ordering::Relaxed);
    }

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

pub struct InputSnapshot {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub escape: bool,
}
