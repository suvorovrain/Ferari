use minifb::{Key, Window};

pub struct Input {
    up: bool,
    left: bool,
    down: bool,
    right: bool,
}

impl Input {
    pub fn new() -> Self {
        Self { up: false, left: false, down: false, right: false }
    }
    pub fn update(&mut self, window: &Window) {
        self.up = window.is_key_down(Key::W);
        self.left = window.is_key_down(Key::A);
        self.down = window.is_key_down(Key::S);
        self.right = window.is_key_down(Key::D);
    }
}
