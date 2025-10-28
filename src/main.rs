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
    // parse atlases
    let tiles_atlas = assets::Atlas::load("assets/tiles/atlas.json").unwrap();
    let entity_atlas = assets::Atlas::load("assets/entities/atlas.json").unwrap();

    // parse game descr
    let game = assets::GameMap::load("input.json").unwrap();

    // init draw
    let input_state = Arc::new(input::InputState::new());
    let running = Arc::new(AtomicBool::new(true));
    let (tx_frame, rx_frame) = bounded::<Vec<u32>>(2);

    // framebuffer (`render <-> draw` connection)
    let mut back_buffer: Vec<u32> = vec![0; LOGIC_WIDTH * LOGIC_HEIGHT];

    {
        let input_state = input_state.clone();
        let running = running.clone();

        thread::spawn(move || {
            draw::run_draw_thread(
                rx_frame,
                input_state,
                running,
                LOGIC_WIDTH,
                LOGIC_HEIGHT,
                UPSCALE,
            );
        });
    }

    // init world_buf
    let world_width = game.size[0] as usize * TILE_SIZE * 2;
    let world_height = game.size[1] as usize * TILE_SIZE * 2;

    let mut world_buf: Vec<u32> = vec![0; world_width * world_height];
    // init render
    let shadow_map: Vec<u8> = vec![0; world_width * world_height];
    let mut render =
        render::Render::new(world_buf, world_height, world_width, entity_atlas, shadow_map);

    // init camera
    let mut camera = world::Camera::new(
        (world_width / 2) as f32,
        (world_height / 2) as f32,
        LOGIC_WIDTH as u16,
        LOGIC_HEIGHT as u16,
    );

    // init time
    let mut time = time::Time::new();

    // prerender
    render.init(&game, &tiles_atlas);
    // game loop
    while running.load(Ordering::Acquire) {
        time.update();

        // test gradient
        // let r = ((time.total).sin() * 127.0 + 128.0) as u32;
        // let g = ((time.total + 2.0).sin() * 127.0 + 128.0) as u32;
        // let b = ((time.total + 4.0).sin() * 127.0 + 128.0) as u32;
        // let color = (r << 16) | (g << 8) | b;

        // for px in back_buffer.iter_mut() {
        //     *px = color;
        // }

        // frame render
        render.render_frame(&camera, &mut back_buffer);

        // draw frame
        if tx_frame.try_send(back_buffer.clone()).is_err() {
            // draw thread busy — пропускаем кадр
        }

        // process input
        let input = input_state.read();
        if input.escape {
            running.store(false, Ordering::Release);
        }

        camera.center_x = camera.center_x + (if input.right { 2.5 } else { 0.0 });
        camera.center_x = camera.center_x + (if input.left { -2.5 } else { 0.0 });
        camera.center_y = camera.center_y + (if input.up { -2.5 } else { 0.0 });
        camera.center_y = camera.center_y + (if input.down { 2.5 } else { 0.0 });

        // fps limit
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    println!("Main loop exited");
}
