use std::time::Instant;

pub struct Time {
    pub delta: f32,        // interval between frames (sec)
    last_instant: Instant, // delta calucalution
}

impl Time {
    pub fn new() -> Self {
        Self { delta: 0.0, last_instant: Instant::now() }
    }

    pub fn update (mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_instant).as_secs_f32();
        self.last_instant = now;
    }

}

