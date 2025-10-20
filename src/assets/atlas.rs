use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use serde_json::Deserialize;

#[derive(Deserialize, Debug)]
struct Frame {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Deserialize, Debug)]
struct Meta {
    image: String,
    tile_size: u32,
    version: u32,
}

#[derive(Deserialize, Debug)]
struct AtlasJson {
    frames: HashMap<String, Frame>,
    meta: Meta,
}

pub struct Frame {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub h: u32,
    pub w: u32,
}

pub struct Atlas {
    pub image: RgbaImage,
    pub frames: HashMap<String, Frame>,
}

impl Atlas{
    fn load(path: &str) -> Result<Self,Error> {
        let image = image.open(path).to_rgba8();
        let mut frames: HashMap<String, Frame> = HashMap::new();
        let atlas

    }
}


