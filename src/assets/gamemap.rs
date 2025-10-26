use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// ============================
/// JSON-level structs
/// ============================

#[derive(Deserialize, Debug, Clone)]
pub struct JsonMob {
    pub x_start: u32,
    pub y_start: u32,
    pub asset: String,

    #[serde(default)]
    pub is_player: bool,

    #[serde(default)]
    pub behaviour: Option<BehaviourJson>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BehaviourJson {
    #[serde(rename = "type")]
    pub behaviour_type: String,

    #[serde(default)]
    pub direction: Option<String>,

    #[serde(default)]
    pub speed: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonObject {
    pub x: u32,
    pub y: u32,
    pub asset: String,

    #[serde(default)]
    pub collidable: bool,

    #[serde(default)]
    pub shadow: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonTile {
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub name: String,

    #[serde(default)]
    pub tile_size: Option<u32>,

    #[serde(default)]
    pub size: Option<[u32; 2]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonMap {
    pub mobs: HashMap<String, JsonMob>,
    pub objects: HashMap<String, JsonObject>,
    pub tiles: HashMap<String, JsonTile>,
    pub meta: Meta,
}

/// ============================
/// Game-level structs
/// ============================

#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub x_start: u32,
    pub y_start: u32,
    pub asset: String,
    pub is_player: bool,
    pub behaviour: Option<Behaviour>,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub asset: String,
    pub collidable: bool,
    pub shadow: bool,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

#[derive(Debug, Clone)]
pub struct Behaviour {
    pub behaviour_type: BehaviourType,
    pub direction: Option<String>,
    pub speed: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BehaviourType {
    Controlled,
    Walker,
    Unknown,
}

/// Game map, as parsed and ready to use
#[derive(Debug)]
pub struct GameMap {
    pub name: String,
    pub tile_size: Option<u32>,
    pub size: Option<[u32; 2]>,
    pub mobs: HashMap<String, Mob>,
    pub objects: HashMap<String, Object>,
    pub tiles: HashMap<String, Tile>,
}

/// ============================
/// Implementation
/// ============================

impl GameMap {
    pub fn load<P: AsRef<Path>>(json_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(json_path)?;
        let reader = BufReader::new(file);
        let map_json: JsonMap = serde_json::from_reader(reader)?;

        let mut mobs = HashMap::new();
        for (name, mob_data) in map_json.mobs {
            let behaviour = mob_data.behaviour.as_ref().map(|b| Behaviour {
                behaviour_type: match b.behaviour_type.as_str() {
                    "controlled" => BehaviourType::Controlled,
                    "walker" => BehaviourType::Walker,
                    _ => BehaviourType::Unknown,
                },
                direction: b.direction.clone(),
                speed: b.speed,
            });

            let mob = Mob {
                name: name.clone(),
                x_start: mob_data.x_start,
                y_start: mob_data.y_start,
                asset: mob_data.asset,
                is_player: mob_data.is_player,
                behaviour,
            };
            mobs.insert(name, mob);
        }

        let mut objects = HashMap::new();
        for (name, obj_data) in map_json.objects {
            let object = Object {
                name: name.clone(),
                x: obj_data.x,
                y: obj_data.y,
                asset: obj_data.asset,
                collidable: obj_data.collidable,
                shadow: obj_data.shadow,
            };
            objects.insert(name, object);
        }

        let mut tiles = HashMap::new();
        for (name, tile_data) in map_json.tiles {
            let tile =
                Tile { name: name.clone(), x: tile_data.x, y: tile_data.y, asset: tile_data.asset };
            tiles.insert(name, tile);
        }

        Ok(GameMap {
            name: map_json.meta.name,
            tile_size: map_json.meta.tile_size,
            size: map_json.meta.size,
            mobs,
            objects,
            tiles,
        })
    }

    pub fn get_mob(&self, name: &str) -> Option<&Mob> {
        self.mobs.get(name)
    }

    pub fn get_object(&self, name: &str) -> Option<&Object> {
        self.objects.get(name)
    }

    pub fn get_tile(&self, name: &str) -> Option<&Tile> {
        self.tiles.get(name)
    }

    pub fn mob_count(&self) -> usize {
        self.mobs.len()
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn iter_mobs(&self) -> impl Iterator<Item = &Mob> {
        self.mobs.values()
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.values()
    }
}

impl Mob {
    pub fn start_position(&self) -> (u32, u32) {
        (self.x_start, self.y_start)
    }
}

impl Object {
    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl Tile {
    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

/// ============================
/// Tests
/// ============================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_with_behaviour() {
        let json_data = r#"
        {
          "mobs": {
            "player": {
              "x_start": 0,
              "y_start": 0,
              "asset": "knight_0_0",
              "is_player": true,
              "behaviour": { "type": "controlled" }
            },
            "mob_1": {
              "x_start": 3,
              "y_start": 3,
              "asset": "imp_20_0",
              "behaviour": {
                "type": "walker",
                "direction": "left",
                "speed": 12.0
              }
            }
          },
          "objects": {
            "obj_1": {
              "x": 2,
              "y": 1,
              "asset": "cactus_long_3_9",
              "collidable": true,
              "shadow": true
            }
          },
          "tiles": {
            "tile_1": { "x": 0, "y": 0, "asset": "dirt_tile_big_0_0" }
          },
          "meta": {
            "name": "demo_map",
            "tile_size": 16,
            "size": [4, 4]
          }
        }"#;

        let game_map: JsonMap = serde_json::from_str(json_data).unwrap();
        assert_eq!(game_map.meta.name, "demo_map");

        let loaded = GameMap::load_from_json_str(json_data).unwrap();
        assert_eq!(loaded.name, "demo_map");
        assert_eq!(loaded.mob_count(), 2);

        let player = loaded.get_mob("player").unwrap();
        assert!(player.is_player);
        assert_eq!(player.behaviour.as_ref().unwrap().behaviour_type, BehaviourType::Controlled);
    }
}

impl GameMap {
    /// Helper for tests — load directly from JSON string
    pub fn load_from_json_str(data: &str) -> Result<Self, Box<dyn Error>> {
        let map_json: JsonMap = serde_json::from_str(data)?;

        let mut mobs = HashMap::new();
        for (name, mob_data) in map_json.mobs {
            let behaviour = mob_data.behaviour.as_ref().map(|b| Behaviour {
                behaviour_type: match b.behaviour_type.as_str() {
                    "controlled" => BehaviourType::Controlled,
                    "walker" => BehaviourType::Walker,
                    _ => BehaviourType::Unknown,
                },
                direction: b.direction.clone(),
                speed: b.speed,
            });

            let mob = Mob {
                name: name.clone(),
                x_start: mob_data.x_start,
                y_start: mob_data.y_start,
                asset: mob_data.asset,
                is_player: mob_data.is_player,
                behaviour,
            };
            mobs.insert(name, mob);
        }

        let mut objects = HashMap::new();
        for (name, obj_data) in map_json.objects {
            let object = Object {
                name: name.clone(),
                x: obj_data.x,
                y: obj_data.y,
                asset: obj_data.asset,
                collidable: obj_data.collidable,
                shadow: obj_data.shadow,
            };
            objects.insert(name, object);
        }

        let mut tiles = HashMap::new();
        for (name, tile_data) in map_json.tiles {
            let tile =
                Tile { name: name.clone(), x: tile_data.x, y: tile_data.y, asset: tile_data.asset };
            tiles.insert(name, tile);
        }

        Ok(GameMap {
            name: map_json.meta.name,
            tile_size: map_json.meta.tile_size,
            size: map_json.meta.size,
            mobs,
            objects,
            tiles,
        })
    }
}
