use ferari::get_five;
use minifb::{Key, Window, WindowOptions};
mod draw;
mod time;

const LOGIC_WIDTH: usize = 200;
const LOGIC_HEIGHT: usize = 200;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;



fn main() {
    // some shit
    let result = get_five();
    println!("Function returned: {}", result);
    
    // read atlas

    // read user game
    
    // init buffers
    let mut buffer: Vec<u32> = vec![0; LOGIC_WIDTH * LOGIC_HEIGHT];

    //init window
    let mut window = Window::new(
        "Ferari - smooth color cycle",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // init Input

    // init Draw
    
    // init Time
    let mut time = time::Time::new();

    // init Camera
    
    // init Initiator

    // init Render

    // init State
    while window.is_open() && !window.is_key_down(Key::Escape) {
        time.update();

        let r = ((time.total).sin() * 127.0 + 128.0) as u32;
        let g = ((time.total + 2.0).sin() * 127.0 + 128.0) as u32;
        let b = ((time.total + 4.0).sin() * 127.0 + 128.0) as u32;

        let color = (r << 16) | (g << 8) | b;

        for i in buffer.iter_mut() {
            *i = color;
        }

        window.update_with_buffer(&buffer, LOGIC_WIDTH, LOGIC_HEIGHT).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }
}
