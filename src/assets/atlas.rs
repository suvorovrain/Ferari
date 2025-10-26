use image::{open, RgbaImage};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Frame from JSON
#[derive(Deserialize, Debug)]
struct JsonFrame {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

/// Meta information from JSON
#[derive(Deserialize, Debug)]
struct Meta {
    image: String,
    tile_size: u32,
    version: u32,
}

/// Parsed JSON file
#[derive(Deserialize, Debug)]
struct AtlasJson {
    frames: HashMap<String, JsonFrame>,
    meta: Meta,
}

/// Frame
#[derive(Debug)]
pub struct Frame {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Parsed atlas
pub struct Atlas {
    pub image: RgbaImage,
    pub frames: HashMap<String, Frame>,
    pub tile_size: u32,
    pub version: u32,
}

impl Atlas {
    /// Load atlas from JSON file
    pub fn load<P: AsRef<Path>>(json_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(json_path)?;
        let reader = BufReader::new(file);
        let atlas_json: AtlasJson = serde_json::from_reader(reader)?;

        let image_path = Path::new(&atlas_json.meta.image);
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

    /// Get frame by name
    pub fn get_frame(&self, name: &str) -> Option<&Frame> {
        self.frames.get(name)
    }

    // Check if atlas contains frame with given name
    pub fn contains_frame(&self, name: &str) -> bool {
        self.frames.contains_key(name)
    }

    /// Get count of frames in atlas
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Iterate through frames
    pub fn iter_frames(&self) -> impl Iterator<Item = &Frame> {
        self.frames.values()
    }

    /// Get image
    pub fn image(&self) -> &RgbaImage {
        &self.image
    }

    /// Get tile size
    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }
}

/*fn main() -> Result<(), Box<dyn Error>> {
    let atlas = Atlas::load("assets/atlas.json")?;

    println!("Loaded {} frames from image: {}", atlas.frame_count(), "atlas.png");

    if let Some(frame) = atlas.get_frame("dirt_tile_big_0_0") {
        println!("Frame 'dirt_tile_big_0_0': {:?}", frame);
        println!("Position: ({}, {})", frame.x, frame.y);
        println!("Size: {}x{}", frame.w, frame.h);
    }

    for frame in atlas.iter_frames() {
        println!("{}: {}x{} at ({}, {})", frame.name, frame.w, frame.h, frame.x, frame.y);
    }

    Ok(())
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parsing() {
        let json_data = r#"
        {
          "frames": {
            "dirt_tile_big_0_0": {
              "x": 1,
              "y": 2,
              "w": 15,
              "h": 15
            },
            "test_tile_1_0": {
              "x": 17,
              "y": 2,
              "w": 15,
              "h": 15
            }
          },
          "meta": {
            "image": "atlas.png",
            "tile_size": 16,
            "version": 1
          }
        }"#;

        let atlas_json: AtlasJson = serde_json::from_str(json_data).unwrap();
        assert_eq!(atlas_json.frames.len(), 2);
        assert_eq!(atlas_json.meta.image, "atlas.png");
        assert_eq!(atlas_json.meta.tile_size, 16);
        assert_eq!(atlas_json.meta.version, 1);

        let frame = &atlas_json.frames["dirt_tile_big_0_0"];
        assert_eq!(frame.x, 1);
        assert_eq!(frame.y, 2);
        assert_eq!(frame.w, 15);
        assert_eq!(frame.h, 15);
    }
}
