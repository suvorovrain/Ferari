use minifb::{Key, Window};
pub struct Draw {
    pub window: Window,
    pub front_buffer: Vec<u32>,
    pub back_buffer: Vec<u32>,
    pub frame_ready: bool,
    pub width: usize,
    pub height: usize,
}

impl Draw {
    pub fn new(
        window: Window,
        fbuf: Vec<u32>,
        bbuf: Vec<u32>,
        ready: bool,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            window: window,
            front_buffer: fbuf,
            back_buffer: bbuf,
            frame_ready: ready,
            width: width,
            height: height,
        }
    }
    pub fn draw(mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if self.frame_ready {
                std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
                self.frame_ready = false;
            }
            self.window.update_with_buffer(&(&self.front_buffer), self.width, self.height).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(5)); // TODO: maybe improve
        }
    }
}
