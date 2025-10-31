use crate::assets::{Atlas, Frame, GameMap, Object, Tile};
use crate::time::Time;
use crate::world::{Camera, Unit};

/// The `Render` struct handles isometric projection rendering with shadow mapping
/// and dynamic entity animation. It maintains world buffers and uses atlas for
/// sprite rendering.
pub struct Render {
    /// Atlas containing entity sprites
    pub entity_atlas: Atlas,
    /// Height of the world buffer in pixels
    pub world_height: usize,
    /// Width of the world buffer in pixels
    pub world_width: usize,
    /// Primary world pixel buffer storing rendered entities
    pub world_buf: Vec<u32>,
    /// Shadow intensity map for shadow calculations
    pub shadow_map: Vec<u8>,
}

impl Render {
    /// Creates a new Render instance.
    ///
    /// # Arguments
    ///
    /// * `world_buf` - Pre-allocated pixel buffer for world rendering
    /// * `height` - Height of the world viewport
    /// * `width` - Width of the world viewport  
    /// * `entity_atlas` - Sprite atlas for entities
    /// * `shadow` - Pre-allocated shadow intensity map
    ///
    /// # Returns
    ///
    /// A new `Render` instance with all values initialized to specified arguments.
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

    /// Initializes the world buffer by rendering static map elements
    ///
    /// Renders tiles and objects from the game map building isometric projection.
    /// Sorts elements by their (x+y) coordinate for depth ordering.
    ///
    /// # Arguments
    ///
    /// * `game` - The game map containing tiles and objects
    /// * `static_atlas` - Sprite atlas for static map elements
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

                self.render_tile(frame, screen_x, screen_y, static_atlas);
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
                self.render_shadow(frame, screen_x, screen_y, static_atlas);
                self.render_object(frame, screen_x, screen_y, static_atlas);
            }
        }
    }

    /// Renders a complete frame
    ///
    /// Combines the pre-rendered world buffer with dynamic entities,
    /// applying camera transformation and time-based animations.
    ///
    /// # Arguments
    ///
    /// * `visible_things` - List of units visible in the current frame
    /// * `camera` - Camera configuration defining viewport and position
    /// * `buf` - Output pixel buffer to render into
    /// * `time` - Current time for animation timing
    pub fn render_frame(
        &mut self,
        visible_things: &[Unit],
        camera: &Camera,
        buf: &mut [u32],
        time: &Time,
    ) {
        // TODO: ADD STATE HANDLING
        let world_w = self.world_width as i32;
        let world_h = self.world_height as i32;

        // left-top
        let half_w = camera.width as i32 / 2;
        let half_h = camera.height as i32 / 2;

        let cam_left = (camera.center_x - camera.width as f32 / 2.0).floor() as i32;
        let cam_top = (camera.center_y - camera.height as f32 / 2.0).floor() as i32;

        let cam_right = camera.center_x as i32 + half_w;
        let cam_bottom = camera.center_y as i32 + half_h;

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
        for (i, unit) in visible_things.iter().enumerate() {
            let name_model = if i == 0 { "knight_0" } else { "imp_20" };
            let period = 0.4;
            let cycles = (time.total / period).floor() as u32;
            let animation_num = if cycles.is_multiple_of(2) { "_0" } else { "_1" };
            let full_name = name_model.to_string() + animation_num;
            if let Some(frame) = self.entity_atlas.get_frame(&full_name) {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                let screen_x =
                    (unit.x as i32 - camera.center_x as i32) + camera.width as i32 / 2 - fw / 2;
                let screen_y =
                    (unit.y as i32 - camera.center_y as i32) + camera.height as i32 / 2 - fh / 2;
                self.render_shadow_unit(frame, screen_x, screen_y, buf, camera);
                self.render_unit(frame, screen_x, screen_y, buf, camera);
            }
        }
    }

    /// Renders a unit to the world buffer
    ///
    /// # Arguments
    ///
    /// * `frame` - Sprite frame to render from the entity atlas
    /// * `screen_x` - X position in screen coordinates (output buffer space)
    /// * `screen_y` - Y position in screen coordinates (output buffer space)  
    /// * `buf` - Output pixel buffer to render into
    /// * `camera` - Camera configuration defining viewport and position
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

                let world_x = (camera.center_x as i32 - camera.width as i32 / 2) + dest_x;
                let world_y = (camera.center_y as i32 - camera.height as i32 / 2) + dest_y;
                let mut brightness = 1.0;

                if world_x >= 0
                    && world_y >= 0
                    && world_x < self.world_width as i32
                    && world_y < self.world_height as i32
                {
                    let shadow_idx = (world_y * self.world_width as i32 + world_x) as usize;
                    let shadow_val = self.shadow_map[shadow_idx] as f32 / 255.0;
                    brightness = 1.0 - 0.6 * shadow_val;
                }

                let [r, g, b, _] = color.0;
                let r = (r as f32 * brightness) as u32;
                let g = (g as f32 * brightness) as u32;
                let b = (b as f32 * brightness) as u32;

                buf[dest_idx] = (0xFF << 24) | (r << 16) | (g << 8) | b;
            }
        }
    }

    /// Renders a tile to the world buffer
    ///
    /// # Arguments
    ///
    /// * `frame` - Sprite frame to render from the entity atlas
    /// * `screen_x` - X position in screen coordinates (output buffer space)
    /// * `screen_y` - Y position in screen coordinates (output buffer space)  
    /// * `atlas` - Sprite atlas for map elements
    fn render_tile(&mut self, frame: &Frame, screen_x: i32, screen_y: i32, atlas: &Atlas) {
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

    /// Renders a static object to the world buffer
    ///
    /// # Arguments
    ///
    /// * `frame` - Sprite frame to render from the entity atlas
    /// * `screen_x` - X position in screen coordinates (output buffer space)
    /// * `screen_y` - Y position in screen coordinates (output buffer space)  
    /// * `atlas` - Sprite atlas for map elements
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

    /// Renders shadow for a static object
    ///
    /// # Arguments
    ///
    /// * `frame` - Sprite frame to render from the entity atlas
    /// * `screen_x` - X position in screen coordinates (output buffer space)
    /// * `screen_y` - Y position in screen coordinates (output buffer space)  
    /// * `atlas` - Sprite atlas for map elements
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
                self.shadow_map[dest_index] = self.shadow_map[dest_index].saturating_add(64);

                let r = ((dst >> 16) & 0xFF) as f32 * 0.5;
                let g = ((dst >> 8) & 0xFF) as f32 * 0.5;
                let b = (dst & 0xFF) as f32 * 0.5;
                self.world_buf[dest_index] =
                    (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }

    /// Renders shadow for a dynamic unit
    ///
    /// # Arguments
    ///
    /// * `frame` - Sprite frame to render from the entity atlas
    /// * `screen_x` - X position in screen coordinates (output buffer space)
    /// * `screen_y` - Y position in screen coordinates (output buffer space)  
    /// * `buf` - Output pixel buffer to render into
    /// * `camera` - Camera configuration defining viewport and position
    fn render_shadow_unit(
        &self,
        frame: &Frame,
        screen_x: i32,
        screen_y: i32,
        buf: &mut [u32],
        camera: &Camera,
    ) {
        let (atlas_w, atlas_h) = self.entity_atlas.image.dimensions();

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

                let color = self.entity_atlas.image.get_pixel(src_x as u32, src_y as u32);
                if color[3] == 0 {
                    continue;
                }

                let height_factor = (frame.h as f32 - dy as f32) * shadow_scale;
                let dest_x = screen_x + dx + (light_dir_x * height_factor) as i32;
                let dest_y = screen_y + dy + (light_dir_y * height_factor) as i32;

                if dest_x < 0
                    || dest_y < 0
                    || dest_x >= camera.width as i32
                    || dest_y >= camera.height as i32
                {
                    continue;
                }

                let dest_idx = (dest_y * camera.width as i32 + dest_x) as usize;
                let dst = buf[dest_idx];

                let r = ((dst >> 16) & 0xFF) as f32 * 0.5;
                let g = ((dst >> 8) & 0xFF) as f32 * 0.5;
                let b = (dst & 0xFF) as f32 * 0.5;
                buf[dest_idx] = (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};
    use std::collections::HashMap;

    fn dummy_atlas(color: [u8; 4]) -> Atlas {
        let mut img = RgbaImage::new(4, 4);
        for y in 0..4 {
            for x in 0..4 {
                img.put_pixel(x, y, Rgba(color));
            }
        }

        let mut frames = HashMap::new();
        frames.insert("dummy".into(), Frame { name: String::new(), x: 0, y: 0, w: 4, h: 4 });

        Atlas { image: img, frames, tile_size: 4, version: 1 }
    }

    fn dummy_camera() -> Camera {
        Camera { center_x: 5.0, center_y: 5.0, width: 10, height: 10 }
    }

    #[test]
    fn test_render_unit_changes_buffer() {
        let atlas = dummy_atlas([255, 0, 0, 255]);
        let mut buf = vec![0; 100];
        let frame = atlas.get_frame("dummy").unwrap();
        let cam = dummy_camera();

        let render = Render::new(vec![0; 100], 10, 10, atlas.clone(), vec![0; 100]);
        render.render_unit(frame, 3, 3, &mut buf, &cam);

        assert!(buf.iter().any(|&p| p != 0), "Buffer must have changed pixels");
    }

    #[test]
    fn test_render_shadow_modifies_shadow_map() {
        let atlas = dummy_atlas([255, 255, 255, 255]);
        let mut render = Render::new(vec![0; 100], 10, 10, atlas.clone(), vec![0; 100]);
        let frame = atlas.get_frame("dummy").unwrap();

        render.render_shadow(frame, 2, 2, &atlas);
        assert!(render.shadow_map.iter().any(|&v| v > 0), "Shadow map must change");
    }
}
