use crate::assets::{Atlas, Frame, GameMap, Object, Tile};
use crate::world::Camera;

/// Represents an entity that can be rendered
#[derive(Clone)]
pub struct RenderableEntity {
    pub x: f32,
    pub y: f32,
    pub sprite_name: String,
}

impl RenderableEntity {
    pub fn new(x: f32, y: f32, sprite_name: String) -> Self {
        Self { x, y, sprite_name }
    }

    pub fn with_sprite(x: f32, y: f32, sprite_name: &str) -> Self {
        Self::new(x, y, sprite_name.to_string())
    }
}

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
    /// Temporary shadow buffer for dynamic objects in current frame
    pub dynamic_shadow_buf: Vec<u8>,
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
            dynamic_shadow_buf: vec![0; height * width],
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
        // Base screen offsets
        let offset_x = self.world_width as i32 / 2;
        let offset_y = (self.world_height as i32 / 2) - (static_atlas.tile_size as i32);

        for tile in tiles {
            if let Some(frame) = static_atlas.get_frame(&tile.asset) {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                // Isometric projection
                let screen_x = (tile.x as i32 - tile.y as i32) * (fw / 2) + offset_x;
                let screen_y = (tile.x as i32 + tile.y as i32) * (fh / 4) + offset_y - (fh / 2);

                self.render_tile(frame, screen_x, screen_y, static_atlas);
            }
        }

        let mut objects: Vec<Object> = (*game).clone().objects.into_values().collect();
        objects.sort_by(|a, b| (a.x + a.y).cmp(&(b.x + b.y)));
        let offset_x = self.world_width as i32 / 2;
        let offset_y = (self.world_height as i32 / 2) - (static_atlas.tile_size as i32);

        const TEXTURE_OFFSET: i32 = 0; // TODO: CURRENTLY DEPENDS ON TEXTURES

        // First render all shadows using references
        for object in &objects {
            if let Some(frame) = static_atlas.get_frame(&object.asset) {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                // Isometric projection
                let screen_x = (object.x as i32 - object.y as i32) * (fw / 2) + offset_x;
                let screen_y = (object.x as i32 + object.y as i32) * (fh / 4) + offset_y
                    - (fh / 2)
                    - TEXTURE_OFFSET;
                self.render_shadow(frame, screen_x, screen_y, static_atlas);
            }
        }

        // Apply blur once after all shadows are rendered
        self.soft_blur_shadows();

        // Then render all objects using references
        for object in &objects {
            if let Some(frame) = static_atlas.get_frame(&object.asset) {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                let screen_x = (object.x as i32 - object.y as i32) * (fw / 2) + offset_x;
                let screen_y = (object.x as i32 + object.y as i32) * (fh / 4) + offset_y
                    - (fh / 2)
                    - TEXTURE_OFFSET;
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
    pub fn render_frame(
        &mut self,
        visible_entities: &[RenderableEntity],
        camera: &Camera,
        buf: &mut [u32],
    ) {
        // TODO: ADD STATE HANDLING
        let world_w = self.world_width as i32;
        let world_h = self.world_height as i32;

        // Left-top
        let half_w = camera.width as i32 / 2;
        let half_h = camera.height as i32 / 2;

        let cam_left = (camera.center_x - camera.width as f32 / 2.0).floor() as i32;
        let cam_top = (camera.center_y - camera.height as f32 / 2.0).floor() as i32;

        let cam_right = camera.center_x as i32 + half_w;
        let cam_bottom = camera.center_y as i32 + half_h;

        // Clear buffer
        for px in buf.iter_mut() {
            *px = 0;
        }

        let view_w = (cam_right - cam_left) as usize;
        let view_h = (cam_bottom - cam_top) as usize;

        // Assert sizes
        assert_eq!(
            buf.len(),
            (camera.width as usize) * (camera.height as usize),
            "Buffer size must match camera viewport"
        );

        // Copy visible world
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

        // Dynamic objects

        // Sort entities by depth for correct rendering order
        let mut sorted_entities: Vec<&RenderableEntity> = visible_entities.iter().collect();
        sorted_entities.sort_by(|a, b| {
            // Primary sort by Y coordinate (higher Y = closer to camera)
            let y_cmp = a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal);
            if y_cmp != std::cmp::Ordering::Equal {
                y_cmp
            } else {
                // Secondary sort by X coordinate if Y is equal
                a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)
            }
        });

        // Reset the shadow temporary buffer
        self.dynamic_shadow_buf.fill(0);

        // Collect all shadow rendering data first
        let mut shadow_render_data = Vec::new();
        for entity in sorted_entities.iter() {
            if let Some(frame) = self.entity_atlas.get_frame(&entity.sprite_name) {
                let fw = frame.w as i32;
                let fh = frame.h as i32;

                let screen_x =
                    (entity.x as i32 - camera.center_x as i32) + camera.width as i32 / 2 - fw / 2;
                let screen_y =
                    (entity.y as i32 - camera.center_y as i32) + camera.height as i32 / 2 - fh;

                shadow_render_data.push((frame.clone(), screen_x, screen_y));
            }
        }

        // Render shadows
        for (frame, screen_x, screen_y) in &shadow_render_data {
            self.render_shadow_unit(frame, *screen_x, *screen_y, buf, camera);
        }

        // Then render all objects
        for (frame, screen_x, screen_y) in &shadow_render_data {
            self.render_unit(frame, *screen_x, *screen_y, buf, camera);
        }
    }

    /// Gets shadow intensity at world coordinates
    pub fn get_shadow_intensity(&self, world_x: i32, world_y: i32) -> f32 {
        if world_x >= 0
            && world_y >= 0
            && world_x < self.world_width as i32
            && world_y < self.world_height as i32
        {
            let shadow_idx = (world_y * self.world_width as i32 + world_x) as usize;
            self.shadow_map[shadow_idx] as f32 / 255.0
        } else {
            0.0
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
                let src_a = color[3] as u32;
                if src_a == 0 {
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

                let shadow_intensity = self.get_shadow_intensity(world_x, world_y);
                let brightness = 1.0 - 0.6 * shadow_intensity;

                let [r, g, b, _] = color.0;
                let src_r = (r as f32 * brightness) as u32;
                let src_g = (g as f32 * brightness) as u32;
                let src_b = (b as f32 * brightness) as u32;

                let dst_pixel = buf[dest_idx];

                let dst_r = (dst_pixel >> 16) & 0xFF;
                let dst_g = (dst_pixel >> 8) & 0xFF;
                let dst_b = dst_pixel & 0xFF;

                let alpha_factor = src_a as f32 / 255.0;
                let inv_alpha_factor = 1.0 - alpha_factor;

                let new_r = (src_r as f32 * alpha_factor + dst_r as f32 * inv_alpha_factor)
                    .min(255.0) as u32;
                let new_g = (src_g as f32 * alpha_factor + dst_g as f32 * inv_alpha_factor)
                    .min(255.0) as u32;
                let new_b = (src_b as f32 * alpha_factor + dst_b as f32 * inv_alpha_factor)
                    .min(255.0) as u32;

                buf[dest_idx] = (0xFF << 24) | (new_r << 16) | (new_g << 8) | new_b;
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
        let light_dir_y = 0.0;
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

                self.shadow_map[dest_index] = self.shadow_map[dest_index].saturating_add(8).min(48);

                let dst = self.world_buf[dest_index];
                let shadow_strength = self.shadow_map[dest_index] as f32 / 255.0;
                let darken_factor = 1.0 - 0.4 * shadow_strength;

                let r = ((dst >> 16) & 0xFF) as f32 * darken_factor;
                let g = ((dst >> 8) & 0xFF) as f32 * darken_factor;
                let b = (dst & 0xFF) as f32 * darken_factor;

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
        &mut self,
        frame: &Frame,
        screen_x: i32,
        screen_y: i32,
        buf: &mut [u32],
        camera: &Camera,
    ) {
        let (atlas_w, atlas_h) = self.entity_atlas.image.dimensions();

        let light_dir_x = 1.0;
        let light_dir_y = 0.0;
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

                let shadow_x = screen_x as f32 + dx as f32 + (light_dir_x * height_factor);
                let shadow_y = screen_y as f32 + dy as f32 + (light_dir_y * height_factor);
                let dest_x = shadow_x.round() as i32;
                let dest_y = shadow_y.round() as i32;

                if dest_x < 0
                    || dest_y < 0
                    || dest_x >= camera.width as i32
                    || dest_y >= camera.height as i32
                {
                    continue;
                }

                // Convert camera coordinates to world coordinates
                let world_x = (camera.center_x as i32 - camera.width as i32 / 2) + dest_x;
                let world_y = (camera.center_y as i32 - camera.height as i32 / 2) + dest_y;

                let dest_idx = (dest_y * camera.width as i32 + dest_x) as usize;

                // Check if there's already a shadow from a static object
                let mut has_shadow = false;
                if world_x >= 0
                    && world_y >= 0
                    && world_x < self.world_width as i32
                    && world_y < self.world_height as i32
                {
                    let shadow_idx = (world_y * self.world_width as i32 + world_x) as usize;
                    // Check static shadow
                    if self.shadow_map[shadow_idx] > 16 {
                        has_shadow = true;
                    }

                    // Check dynamic shadow (in world coordinates)
                    if self.dynamic_shadow_buf[shadow_idx] > 0 {
                        has_shadow = true;
                    } else {
                        // Mark that there's now a dynamic shadow here
                        self.dynamic_shadow_buf[shadow_idx] = 64;
                    }
                }

                if has_shadow {
                    continue;
                }

                let dst = buf[dest_idx];

                const SHADOW_INTNS: f32 = 0.5;
                let r = ((dst >> 16) & 0xFF) as f32 * SHADOW_INTNS;
                let g = ((dst >> 8) & 0xFF) as f32 * SHADOW_INTNS;
                let b = (dst & 0xFF) as f32 * SHADOW_INTNS;

                buf[dest_idx] = (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }

    /// Soft blur for shadow areas only
    pub fn soft_blur_shadows(&mut self) {
        let width = self.world_width as i32;
        let height = self.world_height as i32;
        let mut blurred = self.shadow_map.clone();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = (y * width + x) as usize;

                if self.shadow_map[idx] > 0 {
                    let mut sum = 0u32;
                    let mut count = 0u32;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let sx = x + dx;
                            let sy = y + dy;
                            if sx >= 0 && sy >= 0 && sx < width && sy < height {
                                let sidx = (sy * width + sx) as usize;
                                sum += self.shadow_map[sidx] as u32;
                                count += 1;
                            }
                        }
                    }

                    blurred[idx] = (sum / count) as u8;
                }
            }
        }

        self.shadow_map = blurred;
    }

    pub fn create_entity(&self, x: f32, y: f32, sprite_name: &str) -> RenderableEntity {
        RenderableEntity::with_sprite(x, y, sprite_name)
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
