use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ferari::get_five;
use crossbeam_channel::bounded;

mod draw;
mod input;
mod time;

const LOGIC_WIDTH: usize = 200;
const LOGIC_HEIGHT: usize = 200;
const UPSCALE: usize = 5;

fn main() {
    // some shit
    let result = get_five();
    println!("Function returned: {}", result);

    // init buffer
    let mut back_buffer: Vec<u32> = vec![0; LOGIC_WIDTH * LOGIC_HEIGHT];

    // input shared state
    let input_state = Arc::new(input::InputState::new());

    // init running flag
    let running = Arc::new(AtomicBool::new(true));

    // channel for frames
    let (tx_frame, rx_frame) = bounded::<Vec<u32>>(2);

    // spawn draw thread
    {
        let input_state = input_state.clone();
        let running = running.clone();
        thread::spawn(move || {
            draw::run_draw_thread(rx_frame, input_state, running, LOGIC_WIDTH, LOGIC_HEIGHT,UPSCALE);
        });
    }

    // init Time
    let mut time = time::Time::new();

    while running.load(Ordering::Acquire) {
        time.update();

        let r = ((time.total).sin() * 127.0 + 128.0) as u32;
        let g = ((time.total + 2.0).sin() * 127.0 + 128.0) as u32;
        let b = ((time.total + 4.0).sin() * 127.0 + 128.0) as u32;

        let color = (r << 16) | (g << 8) | b;

        for px in back_buffer.iter_mut() {
            *px = color;
        }

        // send frame
        if tx_frame.try_send(back_buffer.clone()).is_err() {
            // draw thread busy — skip
        }

        // read input state (from draw thread)
        let input = input_state.read();
        if input.escape {
            running.store(false, Ordering::Release);
        }

        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    println!("Main loop exited");
}
