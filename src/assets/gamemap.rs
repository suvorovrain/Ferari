use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Mob from JSON
#[derive(Deserialize, Debug, Clone)]
pub struct JsonMob {
    pub x_start: u32,
    pub y_start: u32,
    pub asset: String,
}

/// Object from JSON
#[derive(Deserialize, Debug, Clone)]
pub struct JsonObject {
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

/// Tile from JSON
#[derive(Deserialize, Debug, Clone)]
pub struct JsonTile {
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

/// Meta information from JSON
#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub name: String,
}

/// Game map parsed from JSON
#[derive(Deserialize, Debug, Clone)]
pub struct JsonMap {
    pub mobs: HashMap<String, JsonMob>,
    pub objects: HashMap<String, JsonObject>,
    pub tiles: HashMap<String, JsonTile>,
    pub meta: Meta,
}

/// Mob
#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub x_start: u32,
    pub y_start: u32,
    pub asset: String,
}

/// Object
#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

/// Tile
#[derive(Debug, Clone)]
pub struct Tile {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

/// Map, provided by user
#[derive(Debug)]
pub struct GameMap {
    pub name: String,
    pub mobs: HashMap<String, Mob>,
    pub objects: HashMap<String, Object>,
    pub tiles: HashMap<String, Tile>,
}

impl GameMap {
    /// Load map from JSON
    pub fn load<P: AsRef<Path>>(json_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(json_path)?;
        let reader = BufReader::new(file);
        let map_json: JsonMap = serde_json::from_reader(reader)?;

        let mut mobs = HashMap::new();
        for (name, mob_data) in map_json.mobs {
            let mob = Mob {
                name: name.clone(),
                x_start: mob_data.x_start,
                y_start: mob_data.y_start,
                asset: mob_data.asset,
            };
            mobs.insert(name, mob);
        }

        let mut objects = HashMap::new();
        for (name, obj_data) in map_json.objects {
            let object =
                Object { name: name.clone(), x: obj_data.x, y: obj_data.y, asset: obj_data.asset };
            objects.insert(name, object);
        }

        let mut tiles = HashMap::new();
        for (name, tile_data) in map_json.tiles {
            let tile =
                Tile { name: name.clone(), x: tile_data.x, y: tile_data.y, asset: tile_data.asset };
            tiles.insert(name, tile);
        }

        Ok(GameMap { name: map_json.meta.name, mobs, objects, tiles })
    }

    /// Get mob by name
    pub fn get_mob(&self, name: &str) -> Option<&Mob> {
        self.mobs.get(name)
    }

    /// Get object by name
    pub fn get_object(&self, name: &str) -> Option<&Object> {
        self.objects.get(name)
    }

    /// Get tile by name
    pub fn get_tile(&self, name: &str) -> Option<&Tile> {
        self.tiles.get(name)
    }

    /// Get count of mobs
    pub fn mob_count(&self) -> usize {
        self.mobs.len()
    }

    /// Get count of objects
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Get count of tiles
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Iterate through mobs
    pub fn iter_mobs(&self) -> impl Iterator<Item = &Mob> {
        self.mobs.values()
    }

    /// Iterate through objects
    pub fn iter_objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    /// Iterate through tiles
    pub fn iter_tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.values()
    }
}

impl Mob {
    /// Get mob's preset coordinates
    pub fn start_position(&self) -> (u32, u32) {
        (self.x_start, self.y_start)
    }
}

impl Object {
    /// Get object's position coordinates
    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl Tile {
    /// Get tile's position coordinates
    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

/* fn main() -> Result<(), Box<dyn Error>> {
    let game_map = GameMap::load("assets/map.json")?;

    println!("Loaded map: '{}'", game_map.name);
    println!(
        "Mobs: {}, Objects: {}, Tiles: {}",
        game_map.mob_count(),
        game_map.object_count(),
        game_map.tile_count()
    );

    for mob in game_map.iter_mobs() {
        println!(
            "Mob '{}': starts at ({}, {}), asset: {}",
            mob.name, mob.x_start, mob.y_start, mob.asset
        );
    }

    for object in game_map.iter_objects() {
        println!(
            "Object '{}': at ({}, {}), asset: {}",
            object.name, object.x, object.y, object.asset
        );
    }

    for tile in game_map.iter_tiles() {
        println!("Tile '{}': at ({}, {}), asset: {}", tile.name, tile.x, tile.y, tile.asset);
    }

    Ok(())
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_parsing() {
        let json_data = r#"
        {
          "mobs": {
            "mob_1": {
              "x_start": 1,
              "y_start": 2,
              "asset": "mob_1"
            },
            "mob_2": {
              "x_start": 3,
              "y_start": 4,
              "asset": "mob_2"
            }
          },
          "objects": {
            "obj_1": {
              "x": 5,
              "y": 6,
              "asset": "obj_1"
            }
          },
          "tiles": {
            "tile_1": {
              "x": 7,
              "y": 8,
              "asset": "tile_1"
            },
            "tile_2": {
              "x": 9,
              "y": 10,
              "asset": "tile_2"
            }
          },
          "meta": {
            "name": "test_map"
          }
        }"#;

        let map_json: JsonMap = serde_json::from_str(json_data).unwrap();

        assert_eq!(map_json.meta.name, "test_map");

        assert_eq!(map_json.mobs.len(), 2);
        let mob_1 = &map_json.mobs["mob_1"];
        assert_eq!(mob_1.x_start, 1);
        assert_eq!(mob_1.y_start, 2);
        assert_eq!(mob_1.asset, "mob_1");

        assert_eq!(map_json.objects.len(), 1);
        let obj_1 = &map_json.objects["obj_1"];
        assert_eq!(obj_1.x, 5);
        assert_eq!(obj_1.y, 6);
        assert_eq!(obj_1.asset, "obj_1");

        assert_eq!(map_json.tiles.len(), 2);
        let tile_2 = &map_json.tiles["tile_2"];
        assert_eq!(tile_2.x, 9);
        assert_eq!(tile_2.y, 10);
        assert_eq!(tile_2.asset, "tile_2");
    }

    #[test]
    fn test_game_map_creation() {
        let json_data = r#"
        {
          "mobs": {
            "test_mob": {
              "x_start": 10,
              "y_start": 20,
              "asset": "goblin"
            }
          },
          "objects": {
            "test_obj": {
              "x": 30,
              "y": 40,
              "asset": "chest"
            }
          },
          "tiles": {
            "test_tile": {
              "x": 50,
              "y": 60,
              "asset": "grass"
            }
          },
          "meta": {
            "name": "test_level"
          }
        }"#;

        let map_json: JsonMap = serde_json::from_str(json_data).unwrap();
        let game_map = GameMap {
            name: map_json.meta.name.clone(),
            mobs: map_json
                .mobs
                .iter()
                .map(|(name, data)| {
                    (
                        name.clone(),
                        Mob {
                            name: name.clone(),
                            x_start: data.x_start,
                            y_start: data.y_start,
                            asset: data.asset.clone(),
                        },
                    )
                })
                .collect(),
            objects: map_json
                .objects
                .iter()
                .map(|(name, data)| {
                    (
                        name.clone(),
                        Object {
                            name: name.clone(),
                            x: data.x,
                            y: data.y,
                            asset: data.asset.clone(),
                        },
                    )
                })
                .collect(),
            tiles: map_json
                .tiles
                .iter()
                .map(|(name, data)| {
                    (
                        name.clone(),
                        Tile {
                            name: name.clone(),
                            x: data.x,
                            y: data.y,
                            asset: data.asset.clone(),
                        },
                    )
                })
                .collect(),
        };

        assert_eq!(game_map.name, "test_level");
        assert_eq!(game_map.mob_count(), 1);
        assert_eq!(game_map.object_count(), 1);
        assert_eq!(game_map.tile_count(), 1);

        let mob = game_map.get_mob("test_mob").unwrap();
        assert_eq!(mob.start_position(), (10, 20));
        assert_eq!(mob.asset, "goblin");
    }
}
