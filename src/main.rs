use ferari::get_five;
use minifb::{Key, Window, WindowOptions};
mod draw;
mod time;

const LOGIC_WIDTH: usize = 200;
const LOGIC_HEIGHT: usize = 200;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;



fn main() {
    let result = get_five();
    println!("Function returned: {}", result);
    let mut buffer: Vec<u32> = vec![0; LOGIC_WIDTH * LOGIC_HEIGHT];

    let mut window = Window::new(
        "Ferari - smooth color cycle",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    let mut t: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        t += 0.01;

        let r = ((t).sin() * 127.0 + 128.0) as u32;
        let g = ((t + 2.0).sin() * 127.0 + 128.0) as u32;
        let b = ((t + 4.0).sin() * 127.0 + 128.0) as u32;

        let color = (r << 16) | (g << 8) | b;

        for i in buffer.iter_mut() {
            *i = color;
        }

        window.update_with_buffer(&buffer, LOGIC_WIDTH, LOGIC_HEIGHT).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }
}
