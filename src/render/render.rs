use crate::assets::{Atlas, Frame, GameMap, Object, Tile};
use crate::world::{Camera, State, Unit};

pub struct Render {
    pub entity_atlas: Atlas,
    pub world_height: usize,
    pub world_width: usize,
    pub world_buf: Vec<u32>,
    pub shadow_map: Vec<u8>,
}

impl Render {
    pub fn new(
        world_buf: Vec<u32>,
        height: usize,
        width: usize,
        entity_atlas: Atlas,
        shadow: Vec<u8>,
    ) -> Self {
        Self {
            world_buf,
            entity_atlas,
            shadow_map: shadow,
            world_height: height,
            world_width: width,
        }
    }

    pub fn init(&mut self, game: &GameMap, static_atlas: &Atlas) {
        let mut tiles: Vec<Tile> = (*game).clone().tiles.into_values().collect();
        tiles.sort_by(|a, b| (a.x + a.y).cmp(&(b.x + b.y)));

        // base screen offsets
        let offset_x = self.world_width as i32 / 2;
        let offset_y = (self.world_height as i32 / 2) - (static_atlas.tile_size as i32);

        for tile in tiles {
            if let Some(frame) = static_atlas.get_frame(&tile.asset) {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                // isometric projection
                let screen_x = (tile.x as i32 - tile.y as i32) * (fw / 2) + offset_x;
                let screen_y = (tile.x as i32 + tile.y as i32) * (fh / 4) + offset_y - (fh / 2);

                self.render_title(&frame, screen_x, screen_y, &static_atlas);
            }
        }

        let mut objects: Vec<Object> = (*game).clone().objects.into_values().collect();
        objects.sort_by(|a, b| (a.x + a.y).cmp(&(b.x + b.y)));
        let offset_x = self.world_width as i32 / 2;
        let offset_y = (self.world_height as i32 / 2) - (static_atlas.tile_size as i32);

        for object in objects {
            if let Some(frame) = static_atlas.get_frame(&object.asset) {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                // isometric projection
                let screen_x = (object.x as i32 - object.y as i32) * (fw / 2) + offset_x;
                let screen_y = (object.x as i32 + object.y as i32) * (fh / 4) + offset_y - (fh / 2);
                self.render_shadow(&frame, screen_x, screen_y, &static_atlas);
                self.render_object(&frame, screen_x, screen_y, &static_atlas);
            }
        }
    }
    pub fn render_frame(&mut self, visible_things: &Vec<Unit>, camera: &Camera, buf: &mut [u32]) {
        // TODO: ADD STATE HANDLING
        let world_w = self.world_width as i32;
        let world_h = self.world_height as i32;

        // left-top
        let half_w = camera.width as i32 / 2;
        let half_h = camera.height as i32 / 2;

        let cam_left = (camera.center_x - camera.width as f32 / 2.0).floor() as i32;
        let cam_top = (camera.center_y - camera.height as f32 / 2.0).floor() as i32;

        let cam_right = (camera.center_x as i32 + half_w);
        let cam_bottom = (camera.center_y as i32 + half_h);

        // clear buf
        for px in buf.iter_mut() {
            *px = 0;
        }

        let view_w = (cam_right - cam_left) as usize;
        let view_h = (cam_bottom - cam_top) as usize;

        // assert sizes
        assert_eq!(
            buf.len(),
            (camera.width as usize) * (camera.height as usize),
            "Buffer size must match camera viewport"
        );

        // copy visible world
        for y in 0..view_h {
            let world_y = cam_top + y as i32;
            if world_y < 0 || world_y >= world_h {
                continue;
            }

            for x in 0..view_w {
                let world_x = cam_left + x as i32;
                if world_x < 0 || world_x >= world_w {
                    continue;
                }

                let world_idx = (world_y * world_w + world_x) as usize;
                let buf_idx = y * camera.width as usize + x;
                buf[buf_idx] = self.world_buf[world_idx];
            }
        }

        // dynamic objects
        for unit in visible_things {
            if let Some(frame) = self.entity_atlas.get_frame("knight_0_0") {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                let screen_x =
                    (unit.x as i32 - camera.center_x as i32) + camera.width as i32 / 2 - fw / 2;
                let screen_y =
                    (unit.y as i32 - camera.center_y as i32) + camera.height as i32 / 2 - fh / 2;

                self.render_unit(&frame, screen_x, screen_y, buf, camera);
            }
        }
    }

    fn render_unit(
        &self,
        frame: &Frame,
        screen_x: i32,
        screen_y: i32,
        buf: &mut [u32],
        camera: &Camera,
    ) {
        let (atlas_w, atlas_h) = self.entity_atlas.image.dimensions();

        for dy in 0..frame.h as i32 {
            for dx in 0..frame.w as i32 {
                let src_x = frame.x as i32 + dx;
                let src_y = frame.y as i32 + dy;

                if src_x < 0 || src_y < 0 || src_x >= atlas_w as i32 || src_y >= atlas_h as i32 {
                    continue;
                }

                let color = self.entity_atlas.image.get_pixel(src_x as u32, src_y as u32);
                if color[3] == 0 {
                    continue;
                }

                let dest_x = screen_x + dx;
                let dest_y = screen_y + dy;

                if dest_x < 0
                    || dest_y < 0
                    || dest_x >= camera.width as i32
                    || dest_y >= camera.height as i32
                {
                    continue;
                }

                let dest_idx = (dest_y * camera.width as i32 + dest_x) as usize;
                let [r, g, b, _] = color.0;
                buf[dest_idx] = (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }

    fn render_title(&mut self, frame: &Frame, screen_x: i32, screen_y: i32, atlas: &Atlas) {
        let (atlas_w, atlas_h) = atlas.image.dimensions();

        for dy in 0..frame.h as i32 {
            for dx in 0..frame.w as i32 {
                let src_x = frame.x as i32 + dx;
                let src_y = frame.y as i32 + dy;

                if src_x < 0 || src_y < 0 || src_x >= atlas_w as i32 || src_y >= atlas_h as i32 {
                    continue;
                }

                let color = atlas.image.get_pixel(src_x as u32, src_y as u32);
                if color[3] == 0 {
                    continue;
                }

                let dest_x = screen_x + dx;
                let dest_y = screen_y + dy;

                if dest_x < 0
                    || dest_y < 0
                    || dest_x >= self.world_width as i32
                    || dest_y >= self.world_height as i32
                {
                    continue;
                }

                let dest_index = (dest_y * self.world_width as i32 + dest_x) as usize;
                let [r, g, b, a] = color.0;
                self.world_buf[dest_index] =
                    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }
    fn render_object(&mut self, frame: &Frame, screen_x: i32, screen_y: i32, atlas: &Atlas) {
        let (atlas_w, atlas_h) = atlas.image.dimensions();

        for dy in 0..frame.h as i32 {
            for dx in 0..frame.w as i32 {
                let src_x = frame.x as i32 + dx;
                let src_y = frame.y as i32 + dy;

                if src_x < 0 || src_y < 0 || src_x >= atlas_w as i32 || src_y >= atlas_h as i32 {
                    continue;
                }

                let color = atlas.image.get_pixel(src_x as u32, src_y as u32);
                if color[3] == 0 {
                    continue;
                }

                let dest_x = screen_x + dx;
                let dest_y = screen_y + dy;

                if dest_x < 0
                    || dest_y < 0
                    || dest_x >= self.world_width as i32
                    || dest_y >= self.world_height as i32
                {
                    continue;
                }

                let dest_index = (dest_y * self.world_width as i32 + dest_x) as usize;
                let [r, g, b, a] = color.0;
                self.world_buf[dest_index] =
                    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }
    fn render_shadow(&mut self, frame: &Frame, screen_x: i32, screen_y: i32, atlas: &Atlas) {
        let (atlas_w, atlas_h) = atlas.image.dimensions();

        let light_dir_x = 1.0;
        let light_dir_y = 0.5;
        let shadow_scale = 0.5;

        for dy in 0..frame.h as i32 {
            for dx in 0..frame.w as i32 {
                let src_x = frame.x as i32 + dx;
                let src_y = frame.y as i32 + dy;

                if src_x < 0 || src_y < 0 || src_x >= atlas_w as i32 || src_y >= atlas_h as i32 {
                    continue;
                }

                let color = atlas.image.get_pixel(src_x as u32, src_y as u32);
                if color[3] == 0 {
                    continue;
                }

                let height_factor = (frame.h as f32 - dy as f32) * shadow_scale;

                let dest_x = screen_x + dx + (light_dir_x * height_factor) as i32;
                let dest_y = screen_y + dy + (light_dir_y * height_factor) as i32;

                if dest_x < 0
                    || dest_y < 0
                    || dest_x >= self.world_width as i32
                    || dest_y >= self.world_height as i32
                {
                    continue;
                }

                let dest_index = (dest_y * self.world_width as i32 + dest_x) as usize;
                let dst = self.world_buf[dest_index];

                let r = ((dst >> 16) & 0xFF) as f32 * 0.5;
                let g = ((dst >> 8) & 0xFF) as f32 * 0.5;
                let b = (dst & 0xFF) as f32 * 0.5;
                self.world_buf[dest_index] =
                    (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }
}
