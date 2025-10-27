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
///
/// # Public fields
///
/// * `behaviour_type` - Type of behaviour
/// * `direction` - Direction for the behaviour
/// * `speed` - Speed value for the behaviour
#[derive(Deserialize, Debug, Clone)]
pub struct BehaviourJson {
    #[serde(rename = "type")]
    pub behaviour_type: String,

    #[serde(default)]
    pub direction: Option<String>,

    #[serde(default)]
    pub speed: Option<f32>,
}

/// Mob data from JSON.
///
/// # Public fields
///
/// * `x_start` - Starting X coordinate of the mob
/// * `y_start` - Starting Y coordinate of the mob
/// * `asset` - Asset identifier for the mob's appearance
/// * `is_player` - Indicates if this mob represents the player character
/// * `behaviour` - Behaviour configuration for the mob
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

/// Object data from JSON.
///
/// # Public fields
///
/// * `x` - X coordinate of the object
/// * `y` - Y coordinate of the object
/// * `asset` - Asset identifier for the object's appearance
/// * `collidable` - Indicates if the object can be collided with
/// * `shadow` - Indicates if the object casts a shadow
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

/// Tile data from JSON.
///
/// # Public fields
///
/// * `x` - X coordinate of the tile
/// * `y` - Y coordinate of the tile
/// * `asset` - Asset identifier for the tile's appearance
#[derive(Deserialize, Debug, Clone)]
pub struct JsonTile {
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

/// Meta information about the game map from JSON.
///
/// # Public fields
///
/// * `name` - Name of the map
/// * `tile_size` - Tile size in pixels
/// * `size` - Map dimensions [width, height]
#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub name: String,

    #[serde(default)]
    pub tile_size: u32,

    #[serde(default)]
    pub size: [u32; 2],
}

/// Complete map structure from JSON.
///
/// # Public fields
///
/// * `mobs` - Mapping of mobs' names to their definitions
/// * `objects` - Mapping of objects' names to their definitions
/// * `tiles` - Mapping of tiles' names to their definitions
/// * `meta` - Map meta information
#[derive(Deserialize, Debug, Clone)]
pub struct JsonMap {
    pub mobs: HashMap<String, JsonMob>,
    pub objects: HashMap<String, JsonObject>,
    pub tiles: HashMap<String, JsonTile>,
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
///
/// # Public fields
///
/// * `behaviour_type` - Type of behaviour
/// * `direction` - Direction for the behaviour
/// * `speed` - Speed value for the behaviour
#[derive(Debug, Clone)]
pub struct Behaviour {
    pub behaviour_type: BehaviourType,
    pub direction: Option<String>,
    pub speed: Option<f32>,
}

/// Mob in the game world.
///
/// # Public fields
///
/// * `name` - Unique identifier for the mob
/// * `x_start` - Starting X coordinate of the mob
/// * `y_start` - Starting Y coordinate of the mob
/// * `asset` - Asset identifier for the mob's appearance
/// * `is_player` - Indicates if this mob represents the player character
/// * `behaviour` - Behaviour configuration for the mob
#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub x_start: u32,
    pub y_start: u32,
    pub asset: String,
    pub is_player: bool,
    pub behaviour: Option<Behaviour>,
}

/// Static object in the game world.
///
/// # Public fields
///
/// * `name` - Unique identifier for the object
/// * `x` - X coordinate of the object
/// * `y` - Y coordinate of the object
/// * `asset` - Asset identifier for the object's appearance
/// * `collidable` - Indicates if the object can be collided with
/// * `shadow` - Indicates if the object casts a shadow
#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub asset: String,
    pub collidable: bool,
    pub shadow: bool,
}

/// Tile in the game world.
///
/// # Public fields
///
/// * `name` - Unique identifier for the tile
/// * `x` - X coordinate of the tile
/// * `y` - Y coordinate of the tile
/// * `asset` - Asset identifier for the tile's appearance
#[derive(Debug, Clone)]
pub struct Tile {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub asset: String,
}

/// Game map, as parsed and ready to use.
///
/// # Public fields
///
/// * `name` - Name of the map
/// * `tile_size` - Tile size in pixels
/// * `size` - Map dimensions [width, height]
/// * `mobs` - Mapping of mobs' names to their definitions
/// * `objects` - Mapping of objects' names to their definitions
/// * `tiles` - Mapping of tiles' names to their definitions
#[derive(Debug)]
pub struct GameMap {
    pub name: String,
    pub tile_size: u32,
    pub size: [u32; 2],
    pub mobs: HashMap<String, Mob>,
    pub objects: HashMap<String, Object>,
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
    pub fn get_tile(&self, name: &str) -> Option<&Tile> {
        self.tiles.get(name)
    }

    /// Returns the number of mobs in the map.
    ///
    /// # Returns
    ///
    /// * `usize` - Count of mobs.
    pub fn mob_count(&self) -> usize {
        self.mobs.len()
    }

    /// Returns the number of objects in the map.
    ///
    /// # Returns
    ///
    /// * `usize` - Count of objects.
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Returns the number of tiles in the map.
    ///
    /// # Returns
    ///
    /// * `usize` - Count of tiles.
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
    pub fn iter_objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    /// Returns an iterator over all tiles in the map.
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Tile>` - Iterator over tile references.
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
        assert_eq!(game_map.size, [4, 4]);

        assert_eq!(game_map.mob_count(), 2);
        assert_eq!(game_map.object_count(), 1);
        assert_eq!(game_map.tile_count(), 16);

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
        assert_eq!(mob_1.x_start, 3);
        assert_eq!(mob_1.y_start, 3);
        assert_eq!(mob_1.asset, "imp_20_0");
        assert_eq!(mob_1.is_player, false);
        assert!(mob_1.behaviour.is_some());

        let mob_behaviour = mob_1.behaviour.as_ref().unwrap();
        assert_eq!(mob_behaviour.behaviour_type, BehaviourType::Walker);
        assert_eq!(mob_behaviour.direction, Some("left".to_string()));
        assert_eq!(mob_behaviour.speed, Some(12.0));
        assert_eq!(mob_1.start_position(), (3, 3));

        let obj_1 = game_map.get_object("obj_1").unwrap();
        assert_eq!(obj_1.name, "obj_1");
        assert_eq!(obj_1.x, 2);
        assert_eq!(obj_1.y, 1);
        assert_eq!(obj_1.asset, "cactus_long_3_9");
        assert_eq!(obj_1.collidable, false);
        assert_eq!(obj_1.shadow, false);
        assert_eq!(obj_1.position(), (2, 1));

        let tile_1 = game_map.get_tile("tile_1").unwrap();
        assert_eq!(tile_1.name, "tile_1");
        assert_eq!(tile_1.x, 0);
        assert_eq!(tile_1.y, 0);
        assert_eq!(tile_1.asset, "dirt_tile_big_0_0");
        assert_eq!(tile_1.position(), (0, 0));

        let tile_16 = game_map.get_tile("tile_16").unwrap();
        assert_eq!(tile_16.name, "tile_16");
        assert_eq!(tile_16.x, 3);
        assert_eq!(tile_16.y, 3);
        assert_eq!(tile_16.asset, "dirt_tile_big_0_0");
        assert_eq!(tile_16.position(), (3, 3));

        let mob_names: Vec<String> = game_map.iter_mobs().map(|m| m.name.clone()).collect();
        assert_eq!(mob_names.len(), 2);
        assert!(mob_names.contains(&"player".to_string()));
        assert!(mob_names.contains(&"mob_1".to_string()));

        let object_names: Vec<String> = game_map.iter_objects().map(|o| o.name.clone()).collect();
        assert_eq!(object_names, vec!["obj_1"]);

        let tile_names: Vec<String> = game_map.iter_tiles().map(|t| t.name.clone()).collect();
        assert_eq!(tile_names.len(), 16);
        assert!(tile_names.contains(&"tile_1".to_string()));
        assert!(tile_names.contains(&"tile_16".to_string()));
    }
}
