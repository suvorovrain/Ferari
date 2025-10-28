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
#[derive(Debug)]
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
#[derive(Debug)]
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
    pub fn contains_frame(&self, name: &str) -> bool {
        self.frames.contains_key(name)
    }

    /// Gets the total number of frames in the atlas.
    ///
    /// # Returns
    ///
    /// * `usize` - The count of frames in the atlas.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Returns an iterator over all frames in the atlas.
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Frame>` - Iterator over frame references.
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
        let atlas = Atlas::load("assets/entities/atlas.json").unwrap();

        assert_eq!(atlas.tile_size, 16);
        assert_eq!(atlas.version, 1);

        assert_eq!(atlas.frame_count(), 12);

        assert!(atlas.contains_frame("knight_0_0"));
        assert!(atlas.contains_frame("knight_0_1"));
        assert!(atlas.contains_frame("knight_1_0"));
        assert!(atlas.contains_frame("knight_1_1"));
        assert!(atlas.contains_frame("imp_20_0"));
        assert!(atlas.contains_frame("imp_20_1"));
        assert!(atlas.contains_frame("imp_21_0"));
        assert!(atlas.contains_frame("imp_21_1"));
        assert!(atlas.contains_frame("ghost_30_0"));
        assert!(atlas.contains_frame("ghost_30_1"));
        assert!(atlas.contains_frame("ghost_31_0"));
        assert!(atlas.contains_frame("ghost_31_1"));

        let knight_frame = atlas.get_frame("knight_0_0").unwrap();
        assert_eq!(knight_frame.name, "knight_0_0");
        assert_eq!(knight_frame.x, 4);
        assert_eq!(knight_frame.y, 12);
        assert_eq!(knight_frame.w, 8);
        assert_eq!(knight_frame.h, 8);

        let knight_frame_1 = atlas.get_frame("knight_0_1").unwrap();
        assert_eq!(knight_frame_1.name, "knight_0_1");
        assert_eq!(knight_frame_1.x, 20);
        assert_eq!(knight_frame_1.y, 12);
        assert_eq!(knight_frame_1.w, 8);
        assert_eq!(knight_frame_1.h, 9);

        let imp_frame = atlas.get_frame("imp_20_0").unwrap();
        assert_eq!(imp_frame.name, "imp_20_0");
        assert_eq!(imp_frame.x, 5);
        assert_eq!(imp_frame.y, 352);
        assert_eq!(imp_frame.w, 8);
        assert_eq!(imp_frame.h, 8);

        let ghost_frame = atlas.get_frame("ghost_30_0").unwrap();
        assert_eq!(ghost_frame.name, "ghost_30_0");
        assert_eq!(ghost_frame.x, 6);
        assert_eq!(ghost_frame.y, 520);
        assert_eq!(ghost_frame.w, 4);
        assert_eq!(ghost_frame.h, 7);

        let ghost_frame_1 = atlas.get_frame("ghost_31_1").unwrap();
        assert_eq!(ghost_frame_1.name, "ghost_31_1");
        assert_eq!(ghost_frame_1.x, 23);
        assert_eq!(ghost_frame_1.y, 537);
        assert_eq!(ghost_frame_1.w, 8);
        assert_eq!(ghost_frame_1.h, 9);

        let mut frame_names: Vec<String> = atlas.iter_frames().map(|f| f.name.clone()).collect();
        frame_names.sort();
        assert_eq!(
            frame_names,
            vec![
                "ghost_30_0",
                "ghost_30_1",
                "ghost_31_0",
                "ghost_31_1",
                "imp_20_0",
                "imp_20_1",
                "imp_21_0",
                "imp_21_1",
                "knight_0_0",
                "knight_0_1",
                "knight_1_0",
                "knight_1_1"
            ]
        );

        assert!(!atlas.image.is_empty());
    }
}
