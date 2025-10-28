use crate::assets::{Atlas, GameMap, Tile, Frame};
use crate::world::State;

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

    pub fn init(&mut self, game: GameMap, static_atlas: Atlas) {
        let mut tiles: Vec<Tile> = game.tiles.into_values().collect();
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
                let screen_y =
                    (tile.x as i32 + tile.y as i32) * (fh / 4) + offset_y - (fh / 2);

                self.draw_tile(&frame, screen_x, screen_y, &static_atlas);
            }
        }
    }

    fn draw_tile(&mut self, frame: &Frame, screen_x: i32, screen_y: i32, atlas: &Atlas) {
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
}
