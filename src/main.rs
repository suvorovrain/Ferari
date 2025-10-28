use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam_channel::bounded;

mod assets;
mod draw;
mod input;
mod render;
mod time;
mod world;

const LOGIC_WIDTH: usize = 200;
const LOGIC_HEIGHT: usize = 200;
const TILE_SIZE: usize = 16;
const UPSCALE: usize = 5;

fn main() {
    // load atlas
    let tiles_atlas = assets::Atlas::load("assets/tiles/atlas.json").unwrap();
    let entity_atlas = assets::Atlas::load("assets/entities/atlas.json").unwrap();

    // load game info
    let game = assets::GameMap::load("input.json").unwrap();

    // init buffer
    let mut back_buffer: Vec<u32> = vec![0; LOGIC_WIDTH * LOGIC_HEIGHT];

    // input shared state
    let input_state = Arc::new(input::InputState::new());

    // init running flag
    let running = Arc::new(AtomicBool::new(true));

    // channel for frames
    let (tx_frame, rx_frame) = bounded::<Vec<u32>>(2);
    let world_width = LOGIC_WIDTH * TILE_SIZE;
    let world_height = LOGIC_HEIGHT * TILE_SIZE + TILE_SIZE;
    // spawn draw thread
    {
        let input_state = input_state.clone();
        let running = running.clone();
        thread::spawn(move || {
            draw::run_draw_thread(
                rx_frame,
                input_state,
                running,
                world_width,
                world_height,
                UPSCALE,
            );
        });
    }

    let mut world_buf: Vec<u32> = vec![0; world_width * world_height];
    let shadow_map: Vec<u8> = vec![0; world_width * world_height];
    println!(
        "world_width={}, world_height={}, buf_len={}",
        world_width,
        world_height,
        world_buf.len()
    );

    let mut render =
        render::Render::new(world_buf, world_height, world_width, entity_atlas, shadow_map);

    render.init(game, tiles_atlas);
    

    // init Time
    let mut time = time::Time::new();
    // prerender
    // world_buf
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
        if tx_frame.try_send(render.world_buf.clone()).is_err() {
            // draw thread busy - skip
        }

        // read input state (from draw thread)
        let input = input_state.read();
        // behaviour::move(&player)
        // state
        // if init
        //      render
        if input.escape {
            running.store(false, Ordering::Release);
        }

        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    println!("Main loop exited");
}
