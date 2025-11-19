use crossbeam_channel::Receiver;
use minifb::{Key, Window, WindowOptions};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::input::InputState;

/// Runs the drawing thread that handles window creation, input processing, and frame rendering.
///
/// This function creates a window and runs a loop that:
/// - Updates the input state from window events
/// - Receives and renders frames
/// - Handles escape key event
/// - Maintains target FPS
///
/// # Arguments
///
/// * `rx_frame` - Receiver channel for receiving frame buffers as vectors of u32 pixels
/// * `input_state` - Shared input state that will be updated with window input events
/// * `running` - Atomic flag to control thread execution and allow shutdown
/// * `width` - Wdth of the frame buffer in pixels
/// * `height` - Height of the frame buffer in pixels
/// * `upscale` - Scaling factor for the window (window size = buffer size * upscale)
///
/// # Behaviour
///
/// The thread will run until `running` is set to false, either by external code
/// or by user pressing Escape or closing the window. The window is created with
/// the title "Ferari" and uses the specified dimensions scaled by the upscale factor.
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
        width * upscale,
        height * upscale, 
        WindowOptions::default()
    ).unwrap();

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
