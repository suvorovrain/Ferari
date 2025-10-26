use minifb::{Key, Window, WindowOptions};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use crossbeam_channel::Receiver;

use crate::input::InputState;

pub fn run_draw_thread(
    rx_frame: Receiver<Vec<u32>>,
    input_state: Arc<InputState>,
    running: Arc<AtomicBool>,
    width: usize,
    height: usize,
    upscale: usize,
) {
    let mut window = Window::new(
        "Ferari",
        width*upscale,
        height*upscale,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    while running.load(Ordering::Acquire) {
        // handle input in this thread
        input_state.update(&window);

        if let Ok(frame) = rx_frame.try_recv() {
            window.update_with_buffer(&frame, width, height).unwrap();
        } else {
            window.update();
        }

        if window.is_key_down(Key::Escape) || !window.is_open() {
            running.store(false, Ordering::Release);
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }

    println!("Draw thread stopped");
}
