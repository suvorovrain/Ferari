use image::{open, RgbaImage};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// ============================
/// JSON-level structs
/// ============================

/// Frame definition from JSON atlas data.
///
/// # Public fields
///
/// * `x` - X coordinate of the frame in the atlas image
/// * `y` - Y coordinate of the frame in the atlas image
/// * `w` - Width of the frame in pixels
/// * `h` - Height of the frame in pixels
#[derive(Deserialize, Debug)]
struct JsonFrame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Meta information about the atlas from JSON.
///
/// # Public fields
///
/// * `image` - Path to the atlas image file
/// * `tile_size` - Size of tiles in the atlas
/// * `version` - Version of the atlas format
#[derive(Deserialize, Debug)]
struct Meta {
    pub image: String,
    pub tile_size: u32,
    pub version: u32,
}

/// Complete parsed JSON atlas data structure.
///
/// # Public fields
///
/// * `frames` - Mapping of frame names to their definitions
/// * `meta` - Meta information about the atlas
#[derive(Deserialize, Debug)]
struct AtlasJson {
    pub frames: HashMap<String, JsonFrame>,
    pub meta: Meta,
}

/// ============================
/// Game-level structs
/// ============================

/// Represents a single frame in the atlas.
///
/// # Public fields
///
/// * `name` - Name identifier of the frame
/// * `x` - X coordinate of the frame in the atlas image
/// * `y` - Y coordinate of the frame in the atlas image
/// * `w` - Width of the frame in pixels
/// * `h` - Height of the frame in pixels
#[derive(Debug)]
pub struct Frame {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Complete atlas containing the image and frame definitions
///
/// # Public fields
///
/// * `image` - The loaded RGBA image data of the atlas
/// * `frames` - Mapping of frame names to frame definitions
/// * `tile_size` - Size of tiles in the atlas
/// * `version` - Version of the atlas
#[derive(Debug)]
pub struct Atlas {
    pub image: RgbaImage,
    pub frames: HashMap<String, Frame>,
    pub tile_size: u32,
    pub version: u32,
}

/// ============================
/// Implementation
/// ============================

impl Atlas {
    /// Loads a texture atlas from a JSON file
    ///
    /// # Arguments
    ///
    /// * `json_path` - Path to the atlas JSON file
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - Ok(Atlas) if successful, Err otherwise
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

    /// Retrieves a frame by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the frame to retrieve
    ///
    /// # Returns
    ///
    /// * `Option<&Frame>` - Some(&Frame) if frame exists, None otherwise
    pub fn get_frame(&self, name: &str) -> Option<&Frame> {
        self.frames.get(name)
    }

    /// Checks if the atlas contains a frame with the given name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the frame to check for
    ///
    /// # Returns
    ///
    /// * `bool` - true if frame exists, false otherwise
    pub fn contains_frame(&self, name: &str) -> bool {
        self.frames.contains_key(name)
    }

    /// Gets the total number of frames in the atlas
    ///
    /// # Returns
    ///
    /// * `usize` - The count of frames in the atlas
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Returns an iterator over all frames in the atlas
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Frame>` - Iterator over frame references
    pub fn iter_frames(&self) -> impl Iterator<Item = &Frame> {
        self.frames.values()
    }
}

/// ============================
/// Tests
/// ============================


#[cfg(test)]
mod tests {
    use super::*;

    // Test atlas JSON parsing on example
    #[test]
    fn test_load_entities_atlas() {
        let atlas = Atlas::load("assets/entities/atlas.json").unwrap();
        
        assert_eq!(atlas.tile_size, 16);
        assert_eq!(atlas.version, 1);
        
        assert_eq!(atlas.frame_count(), 2);
        
        assert!(atlas.contains_frame("knight_0_0"));
        assert!(atlas.contains_frame("imp_20_0"));
        
        let knight_frame = atlas.get_frame("knight_0_0").unwrap();
        assert_eq!(knight_frame.name, "knight_0_0");
        assert_eq!(knight_frame.x, 4);
        assert_eq!(knight_frame.y, 14);
        assert_eq!(knight_frame.w, 10);
        assert_eq!(knight_frame.h, 10);
        
        let imp_frame = atlas.get_frame("imp_20_0").unwrap();
        assert_eq!(imp_frame.name, "imp_20_0");
        assert_eq!(imp_frame.x, 5);
        assert_eq!(imp_frame.y, 354);
        assert_eq!(imp_frame.w, 10);
        assert_eq!(imp_frame.h, 10);
        
        let mut frame_names: Vec<String> = atlas.iter_frames().map(|f| f.name.clone()).collect();
        frame_names.sort();
        assert_eq!(frame_names, vec!["imp_20_0", "knight_0_0"]);
        
        assert!(!atlas.image.is_empty());
    }
}
