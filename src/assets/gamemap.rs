use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// ============================
// JSON-level structs
// ============================

/// Behaviour data from JSON.
#[derive(Deserialize, Debug, Clone)]
pub struct BehaviourJson {
    /// Type of behaviour
    #[serde(rename = "type")]
    pub behaviour_type: String,

    /// Direction for the behaviour
    #[serde(default)]
    pub direction: Option<String>,

    /// Speed value for the behaviour
    #[serde(default)]
    pub speed: Option<f32>,
}

/// Mob data from JSON.
#[derive(Deserialize, Debug, Clone)]
pub struct JsonMob {
    /// Starting X coordinate of the mob
    pub x_start: u32,
    /// Starting Y coordinate of the mob
    pub y_start: u32,
    /// Asset identifier for the mob's appearance
    pub asset: String,

    /// Indicates if this mob represents the player character
    #[serde(default)]
    pub is_player: bool,

    /// Behaviour configuration for the mob
    #[serde(default)]
    pub behaviour: Option<BehaviourJson>,
}

/// Object data from JSON.
#[derive(Deserialize, Debug, Clone)]
pub struct JsonObject {
    /// X coordinate of the object
    pub x: u32,
    /// Y coordinate of the object
    pub y: u32,
    /// Asset identifier for the object's appearance
    pub asset: String,

    /// Indicates if the object can be collided with
    #[serde(default)]
    pub collidable: bool,

    /// Indicates if the object casts a shadow
    #[serde(default)]
    pub shadow: bool,
}

/// Tile data from JSON.
#[derive(Deserialize, Debug, Clone)]
pub struct JsonTile {
    /// X coordinate of the tile
    pub x: u32,
    /// Y coordinate of the tile
    pub y: u32,
    /// Asset identifier for the tile's appearance
    pub asset: String,
}

/// Meta information about the game map from JSON.
#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    /// Name of the map
    pub name: String,

    /// Tile size in pixels
    #[serde(default)]
    pub tile_size: u32,

    /// Map dimensions [width, height]
    #[serde(default)]
    pub size: [u32; 2],
}

/// Complete map structure from JSON.
#[derive(Deserialize, Debug, Clone)]
pub struct JsonMap {
    /// Mapping of mobs' names to their definitions
    pub mobs: HashMap<String, JsonMob>,
    /// Mapping of objects' names to their definitions
    pub objects: HashMap<String, JsonObject>,
    /// Mapping of tiles' names to their definitions
    pub tiles: HashMap<String, JsonTile>,
    /// Map meta information
    pub meta: Meta,
}

// ============================
// Game-level structs
// ============================

/// Types of behaviours that mobs can show.
#[derive(Debug, Clone, PartialEq)]
pub enum BehaviourType {
    /// Player-controlled behaviour
    Controlled,
    /// Autonomous walking behaviour
    Walker,
    /// Unknown behaviour type
    Unknown,
}

/// Processed behaviour data for game logic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Behaviour {
    /// Type of behaviour
    pub behaviour_type: BehaviourType,
    /// Direction for the behaviour
    pub direction: Option<String>,
    /// Speed value for the behaviour
    pub speed: Option<f32>,
}

/// Mob in the game world.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Mob {
    /// Unique identifier for the mob
    pub name: String,
    /// Starting X coordinate of the mob
    pub x_start: u32,
    /// Starting Y coordinate of the mob
    pub y_start: u32,
    /// Asset identifier for the mob's appearance
    pub asset: String,
    /// Indicates if this mob represents the player character
    pub is_player: bool,
    /// Behaviour configuration for the mob
    pub behaviour: Option<Behaviour>,
}

/// Static object in the game world.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Object {
    /// Unique identifier for the object
    pub name: String,
    /// X coordinate of the object
    pub x: u32,
    /// Y coordinate of the object
    pub y: u32,
    /// Asset identifier for the object's appearance
    pub asset: String,
    /// Indicates if the object can be collided with
    pub collidable: bool,
    /// Indicates if the object casts a shadow
    pub shadow: bool,
}

/// Tile in the game world.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tile {
    /// Unique identifier for the tile
    pub name: String,
    /// X coordinate of the tile
    pub x: u32,
    /// Y coordinate of the tile
    pub y: u32,
    /// Asset identifier for the tile's appearance
    pub asset: String,
}

/// Game map, as parsed and ready to use.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GameMap {
    /// Name of the map
    pub name: String,
    /// Tile size in pixels
    pub tile_size: u32,
    /// Map dimensions [width, height]
    pub size: [u32; 2],
    /// Mapping of mobs' names to their definitions
    pub mobs: HashMap<String, Mob>,
    /// Mapping of objects' names to their definitions
    pub objects: HashMap<String, Object>,
    /// Mapping of tiles' names to their definitions
    pub tiles: HashMap<String, Tile>,
}

// ============================
// Implementation
// ============================

impl GameMap {
    /// Loads and parses a game map from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `json_path` - Path to the JSON file containing map data
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - Parsed GameMap on success, error on failure.
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

    /// Retrieves a mob by name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the mob to retrieve
    ///
    /// # Returns
    ///
    /// * `Option<&Mob>` - Reference to the mob if found, None otherwise.
    #[allow(dead_code)]
    pub fn get_mob(&self, name: &str) -> Option<&Mob> {
        self.mobs.get(name)
    }

    /// Retrieves an object by name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the object to retrieve
    ///
    /// # Returns
    ///
    /// * `Option<&Object>` - Reference to the object if found, None otherwise.
    #[allow(dead_code)]
    pub fn get_object(&self, name: &str) -> Option<&Object> {
        self.objects.get(name)
    }

    /// Retrieves a tile by name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the tile to retrieve
    ///
    /// # Returns
    ///
    /// * `Option<&Tile>` - Reference to the tile if found, None otherwise.
    #[allow(dead_code)]
    pub fn get_tile(&self, name: &str) -> Option<&Tile> {
        self.tiles.get(name)
    }

    /// Returns the number of mobs in the map.
    ///
    /// # Returns
    ///
    /// * `usize` - Count of mobs.
    #[allow(dead_code)]
    pub fn mob_count(&self) -> usize {
        self.mobs.len()
    }

    /// Returns the number of objects in the map.
    ///
    /// # Returns
    ///
    /// * `usize` - Count of objects.
    #[allow(dead_code)]
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Returns the number of tiles in the map.
    ///
    /// # Returns
    ///
    /// * `usize` - Count of tiles.
    #[allow(dead_code)]
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Returns an iterator over all mobs in the map.
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Mob>` - Iterator over mob references.
    pub fn iter_mobs(&self) -> impl Iterator<Item = &Mob> {
        self.mobs.values()
    }

    /// Returns an iterator over all objects in the map.
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Object>` - Iterator over object references.
    #[allow(dead_code)]
    pub fn iter_objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    /// Returns an iterator over all tiles in the map.
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Tile>` - Iterator over tile references.
    #[allow(dead_code)]
    pub fn iter_tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.values()
    }
}

impl Mob {
    /// Returns the start position of the mob.
    ///
    /// # Returns
    ///
    /// * `(u32, u32)` - Tuple of (x, y) coordinates.
    #[allow(dead_code)]
    pub fn start_position(&self) -> (u32, u32) {
        (self.x_start, self.y_start)
    }
}

impl Object {
    /// Returns the position of the object.
    ///
    /// # Returns
    ///
    /// * `(u32, u32)` - Tuple of (x, y) coordinates.
    #[allow(dead_code)]
    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl Tile {
    /// Returns the position of the tile.
    ///
    /// # Returns
    ///
    /// * `(u32, u32)` - Tuple of (x, y) coordinates.
    #[allow(dead_code)]
    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

// ============================
// Tests
// ============================

#[cfg(test)]
mod tests {
    use super::*;

    // Test game map parsing on example
    #[test]
    fn test_load_game_map() {
        let game_map = GameMap::load("input.json").unwrap();

        assert_eq!(game_map.name, "demo_map");
        assert_eq!(game_map.tile_size, 16);
        assert_eq!(game_map.size, [25, 25]);

        assert_eq!(game_map.mob_count(), 6);
        assert_eq!(game_map.object_count(), 3);
        assert_eq!(game_map.tile_count(), 625);

        let player = game_map.get_mob("player").unwrap();
        assert_eq!(player.name, "player");
        assert_eq!(player.x_start, 0);
        assert_eq!(player.y_start, 0);
        assert_eq!(player.asset, "knight_0_0");
        assert_eq!(player.is_player, true);
        assert!(player.behaviour.is_some());

        let player_behaviour = player.behaviour.as_ref().unwrap();
        assert_eq!(player_behaviour.behaviour_type, BehaviourType::Controlled);
        assert_eq!(player_behaviour.direction, None);
        assert_eq!(player_behaviour.speed, None);
        assert_eq!(player.start_position(), (0, 0));

        let mob_1 = game_map.get_mob("mob_1").unwrap();
        assert_eq!(mob_1.name, "mob_1");
        assert_eq!(mob_1.x_start, 440);
        assert_eq!(mob_1.y_start, 470);
        assert_eq!(mob_1.asset, "imp_20_0");
        assert_eq!(mob_1.is_player, false);
        assert!(mob_1.behaviour.is_some());

        let mob_1_behaviour = mob_1.behaviour.as_ref().unwrap();
        assert_eq!(mob_1_behaviour.behaviour_type, BehaviourType::Walker);
        assert_eq!(mob_1_behaviour.direction, Some("left".to_string()));
        assert_eq!(mob_1_behaviour.speed, Some(0.5));
        assert_eq!(mob_1.start_position(), (440, 470));

        let mob_2 = game_map.get_mob("mob_2").unwrap();
        assert_eq!(mob_2.name, "mob_2");
        assert_eq!(mob_2.x_start, 400);
        assert_eq!(mob_2.y_start, 460);
        assert_eq!(mob_2.asset, "ghost_30_0");
        assert_eq!(mob_2.is_player, false);
        assert!(mob_2.behaviour.is_some());

        let mob_2_behaviour = mob_2.behaviour.as_ref().unwrap();
        assert_eq!(mob_2_behaviour.behaviour_type, BehaviourType::Walker);
        assert_eq!(mob_2_behaviour.direction, Some("right".to_string()));
        assert_eq!(mob_2_behaviour.speed, Some(0.42));
        assert_eq!(mob_2.start_position(), (400, 460));

        let mob_4 = game_map.get_mob("mob_4").unwrap();
        assert_eq!(mob_4.name, "mob_4");
        assert_eq!(mob_4.x_start, 420);
        assert_eq!(mob_4.y_start, 470);
        assert_eq!(mob_4.asset, "imp_20_0");
        assert_eq!(mob_4.is_player, false);
        assert!(mob_4.behaviour.is_some());

        let mob_5 = game_map.get_mob("mob_5").unwrap();
        assert_eq!(mob_5.name, "mob_5");
        assert_eq!(mob_5.x_start, 493);
        assert_eq!(mob_5.y_start, 470);
        assert_eq!(mob_5.asset, "imp_20_0");
        assert_eq!(mob_5.is_player, false);
        assert!(mob_5.behaviour.is_some());

        let mob_6 = game_map.get_mob("mob_6").unwrap();
        assert_eq!(mob_6.name, "mob_6");
        assert_eq!(mob_6.x_start, 540);
        assert_eq!(mob_6.y_start, 470);
        assert_eq!(mob_6.asset, "imp_20_0");
        assert_eq!(mob_6.is_player, false);
        assert!(mob_6.behaviour.is_some());

        let obj_1 = game_map.get_object("obj_1").unwrap();
        assert_eq!(obj_1.name, "obj_1");
        assert_eq!(obj_1.x, 2);
        assert_eq!(obj_1.y, 1);
        assert_eq!(obj_1.asset, "cactus_long_3_9");
        assert_eq!(obj_1.collidable, false);
        assert_eq!(obj_1.shadow, false);
        assert_eq!(obj_1.position(), (2, 1));

        let obj_2 = game_map.get_object("obj_2").unwrap();
        assert_eq!(obj_2.name, "obj_2");
        assert_eq!(obj_2.x, 4);
        assert_eq!(obj_2.y, 14);
        assert_eq!(obj_2.asset, "fence_rising_11_10");
        assert_eq!(obj_2.collidable, false);
        assert_eq!(obj_2.shadow, false);
        assert_eq!(obj_2.position(), (4, 14));

        let obj_3 = game_map.get_object("obj_3").unwrap();
        assert_eq!(obj_3.name, "obj_3");
        assert_eq!(obj_3.x, 8);
        assert_eq!(obj_3.y, 15);
        assert_eq!(obj_3.asset, "fence_falling_10_10");
        assert_eq!(obj_3.collidable, false);
        assert_eq!(obj_3.shadow, false);
        assert_eq!(obj_3.position(), (8, 15));

        let tile_1 = game_map.get_tile("tile_1").unwrap();
        assert_eq!(tile_1.name, "tile_1");
        assert_eq!(tile_1.x, 0);
        assert_eq!(tile_1.y, 0);
        assert_eq!(tile_1.asset, "grass_tile_big_0_1");
        assert_eq!(tile_1.position(), (0, 0));

        let tile_625 = game_map.get_tile("tile_625").unwrap();
        assert_eq!(tile_625.name, "tile_625");
        assert_eq!(tile_625.x, 24);
        assert_eq!(tile_625.y, 24);
        assert_eq!(tile_625.asset, "grass_tile_big_0_1");
        assert_eq!(tile_625.position(), (24, 24));

        let mob_names: Vec<String> = game_map.iter_mobs().map(|m| m.name.clone()).collect();
        assert_eq!(mob_names.len(), 6); // was 3
        assert!(mob_names.contains(&"player".to_string()));
        assert!(mob_names.contains(&"mob_1".to_string()));
        assert!(mob_names.contains(&"mob_2".to_string()));
        assert!(mob_names.contains(&"mob_4".to_string()));
        assert!(mob_names.contains(&"mob_5".to_string()));
        assert!(mob_names.contains(&"mob_6".to_string()));

        let object_names: Vec<String> = game_map.iter_objects().map(|o| o.name.clone()).collect();
        assert_eq!(object_names.len(), 3);
        assert!(object_names.contains(&"obj_1".to_string()));
        assert!(object_names.contains(&"obj_2".to_string()));
        assert!(object_names.contains(&"obj_3".to_string()));

        let tile_names: Vec<String> = game_map.iter_tiles().map(|t| t.name.clone()).collect();
        assert_eq!(tile_names.len(), 625);
        assert!(tile_names.contains(&"tile_1".to_string()));
        assert!(tile_names.contains(&"tile_625".to_string()));
    }
}
