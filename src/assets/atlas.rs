use image::{open, RgbaImage};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

// ============================
// JSON-level structs
// ============================

/// Frame definition from JSON atlas data.
#[derive(Deserialize, Debug)]
struct JsonFrame {
    /// X coordinate of the frame in the atlas image
    pub x: u32,
    /// Y coordinate of the frame in the atlas image
    pub y: u32,
    /// Width of the frame in pixels
    pub w: u32,
    /// Height of the frame in pixels
    pub h: u32,
}

/// Meta information about the atlas from JSON.
#[derive(Deserialize, Debug)]
struct Meta {
    /// Path to the atlas image file
    pub image: String,
    /// Size of tiles in the atlas
    pub tile_size: u32,
    /// Version of the atlas format
    pub version: u32,
}

/// Complete parsed JSON atlas data structure.
#[derive(Deserialize, Debug)]
struct AtlasJson {
    /// Mapping of frame names to their definitions
    pub frames: HashMap<String, JsonFrame>,
    /// Meta information about the atlas
    pub meta: Meta,
}

// ============================
// Game-level structs
// ============================

/// Represents a single frame in the atlas.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Frame {
    /// Name identifier of the frame
    pub name: String,
    /// X coordinate of the frame in the atlas image
    pub x: u32,
    /// Y coordinate of the frame in the atlas image
    pub y: u32,
    /// Width of the frame in pixels
    pub w: u32,
    /// Height of the frame in pixels
    pub h: u32,
}

/// Complete atlas containing the image and frame definitions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Atlas {
    /// The loaded RGBA image data of the atlas
    pub image: RgbaImage,
    /// Mapping of frame names to frame definitions
    pub frames: HashMap<String, Frame>,
    /// Size of tiles in the atlas
    pub tile_size: u32,
    /// Version of the atlas
    pub version: u32,
}

// ============================
// Implementation
// ============================

impl Atlas {
    /// Loads a texture atlas from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `json_path` - Path to the atlas JSON file
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - Ok(Atlas) if successful, Err otherwise.
    pub fn load<P: AsRef<Path>>(json_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(&json_path)?;

        let reader = BufReader::new(file);
        let atlas_json: AtlasJson = serde_json::from_reader(reader)?;

        let image_path = json_path
            .as_ref()
            .parent()
            .map(|dir| dir.join(&atlas_json.meta.image))
            .unwrap_or_else(|| PathBuf::from(&atlas_json.meta.image));

        let image = open(image_path)?.to_rgba8();

        let mut frames = HashMap::new();

        for (name, json_frame) in atlas_json.frames {
            let frame = Frame {
                name: name.clone(),
                x: json_frame.x,
                y: json_frame.y,
                w: json_frame.w,
                h: json_frame.h,
            };
            frames.insert(name, frame);
        }

        Ok(Atlas {
            image,
            frames,
            tile_size: atlas_json.meta.tile_size,
            version: atlas_json.meta.version,
        })
    }

    /// Retrieves a frame by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the frame to retrieve
    ///
    /// # Returns
    ///
    /// * `Option<&Frame>` - Some(&Frame) if frame exists, None otherwise.
    pub fn get_frame(&self, name: &str) -> Option<&Frame> {
        self.frames.get(name)
    }

    /// Checks if the atlas contains a frame with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the frame to check for
    ///
    /// # Returns
    ///
    /// * `bool` - true if frame exists, false otherwise.
    #[allow(dead_code)]
    pub fn contains_frame(&self, name: &str) -> bool {
        self.frames.contains_key(name)
    }

    /// Gets the total number of frames in the atlas.
    ///
    /// # Returns
    ///
    /// * `usize` - The count of frames in the atlas.
    #[allow(dead_code)]
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Returns an iterator over all frames in the atlas.
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Frame>` - Iterator over frame references.
    #[allow(dead_code)]
    pub fn iter_frames(&self) -> impl Iterator<Item = &Frame> {
        self.frames.values()
    }
}

// ============================
// Tests
// ============================

#[cfg(test)]
mod tests {
    use super::*;

    // Test atlas JSON parsing on example
    #[test]
    fn test_load_entities_atlas() {
        let atlas = Atlas::load("assets/tiles/atlas.json").unwrap();

        assert_eq!(atlas.tile_size, 16);
        assert_eq!(atlas.version, 1);

        assert_eq!(atlas.frame_count(), 11);

        assert!(atlas.contains_frame("dirt_tile_big_0_0"));
        assert!(atlas.contains_frame("grass_tile_big_0_1"));
        assert!(atlas.contains_frame("rock_tile_big_0_2"));
        assert!(atlas.contains_frame("sand_tile_big_0_3"));
        assert!(atlas.contains_frame("dirt_tile_small_1_0"));
        assert!(atlas.contains_frame("grass_tile_small_1_1"));
        assert!(atlas.contains_frame("rock_tile_small_1_2"));
        assert!(atlas.contains_frame("sand_tile_small_1_3"));
        assert!(atlas.contains_frame("cactus_long_3_9"));
        assert!(atlas.contains_frame("fence_rising_11_10"));
        assert!(atlas.contains_frame("fence_falling_10_10"));

        let dirt_big_frame = atlas.get_frame("dirt_tile_big_0_0").unwrap();
        assert_eq!(dirt_big_frame.name, "dirt_tile_big_0_0");
        assert_eq!(dirt_big_frame.x, 0);
        assert_eq!(dirt_big_frame.y, 1);
        assert_eq!(dirt_big_frame.w, 16);
        assert_eq!(dirt_big_frame.h, 16);

        let grass_big_frame = atlas.get_frame("grass_tile_big_0_1").unwrap();
        assert_eq!(grass_big_frame.name, "grass_tile_big_0_1");
        assert_eq!(grass_big_frame.x, 16);
        assert_eq!(grass_big_frame.y, 0);
        assert_eq!(grass_big_frame.w, 16);
        assert_eq!(grass_big_frame.h, 17);

        let dirt_small_frame = atlas.get_frame("dirt_tile_small_1_0").unwrap();
        assert_eq!(dirt_small_frame.name, "dirt_tile_small_1_0");
        assert_eq!(dirt_small_frame.x, 0);
        assert_eq!(dirt_small_frame.y, 33);
        assert_eq!(dirt_small_frame.w, 16);
        assert_eq!(dirt_small_frame.h, 16);

        let cactus_frame = atlas.get_frame("cactus_long_3_9").unwrap();
        assert_eq!(cactus_frame.name, "cactus_long_3_9");
        assert_eq!(cactus_frame.x, 148);
        assert_eq!(cactus_frame.y, 52);
        assert_eq!(cactus_frame.w, 10);
        assert_eq!(cactus_frame.h, 16);

        let fence_rising_frame = atlas.get_frame("fence_rising_11_10").unwrap();
        assert_eq!(fence_rising_frame.name, "fence_rising_11_10");
        assert_eq!(fence_rising_frame.x, 162);
        assert_eq!(fence_rising_frame.y, 153);
        assert_eq!(fence_rising_frame.w, 16);
        assert_eq!(fence_rising_frame.h, 16);

        let fence_falling_frame = atlas.get_frame("fence_falling_10_10").unwrap();
        assert_eq!(fence_falling_frame.name, "fence_falling_10_10");
        assert_eq!(fence_falling_frame.x, 162);
        assert_eq!(fence_falling_frame.y, 136);
        assert_eq!(fence_falling_frame.w, 16);
        assert_eq!(fence_falling_frame.h, 16);

        let mut frame_names: Vec<String> = atlas.iter_frames().map(|f| f.name.clone()).collect();
        frame_names.sort();
        assert_eq!(
            frame_names,
            vec![
                "cactus_long_3_9",
                "dirt_tile_big_0_0",
                "dirt_tile_small_1_0",
                "fence_falling_10_10",
                "fence_rising_11_10",
                "grass_tile_big_0_1",
                "grass_tile_small_1_1",
                "rock_tile_big_0_2",
                "rock_tile_small_1_2",
                "sand_tile_big_0_3",
                "sand_tile_small_1_3"
            ]
        );

        assert!(!atlas.image.is_empty());
    }
}
