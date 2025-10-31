use crate::assets::GameMap;

/// Represents the current game state containing all units.
///
/// The `State` struct manages the player unit and all mob units in the game,
/// tracking their positions and movement speeds for game simulation.
#[derive(Debug, Default)]
pub struct State {
    /// The player-controlled unit
    pub player: Unit,
    /// Collection of all non-player mobile units
    pub mobs: Vec<Unit>,
}

/// Represents a unit entity in the game world with position and movement capabilities.
///
/// Units can be either player-controlled or game-controlled mobs. Each unit has
/// a position in 2D space and speed components for movement simulation.
#[derive(Debug, Clone, Default)]
pub struct Unit {
    /// X-coordinate position in the game world
    pub x: f32,
    /// Y-coordinate position in the game world
    pub y: f32,
    /// Horizontal movement speed
    pub x_speed: f32,
    /// Vertical movement speed
    pub y_speed: f32,
}

impl Unit {
    /// Creates a new `Unit` with the specified position and movement parameters.
    ///
    /// # Arguments
    ///
    /// * `x` - Initial X-coordinate position
    /// * `y` - Initial Y-coordinate position
    /// * `x_speed` - Initial horizontal movement speed
    /// * `y_speed` - Initial vertical movement speed
    ///
    /// # Returns
    ///
    /// A new `Unit` instance with the specified properties.
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, x_speed: f32, y_speed: f32) -> Self {
        Self { x, y, x_speed, y_speed }
    }
}

impl State {
    /// Creates a new `State` by getting unit data from a `GameMap`.
    ///
    /// This constructor processes all units defined in the game map, identifying
    /// the player unit and initializing all mob units with their starting positions
    /// and movement behaviors.
    ///
    /// # Arguments
    ///
    /// * `game_map` - Reference to the `GameMap` containing unit definitions
    ///
    /// # Returns
    ///
    /// A new `State` instance with:
    /// - Player unit initialized from the mob marked as `is_player`
    /// - All other mobs initialized with their respective behaviors and speeds
    ///
    /// # Behavior
    ///
    /// - The player unit is given fixed movement speeds (10.0 in both directions)
    /// - Mob units derive their movement from behavior definitions:
    ///   - "right": positive x_speed
    ///   - "left": negative x_speed  
    ///   - "up": negative y_speed
    ///   - "down": positive y_speed
    ///   - Mobs without behavior definitions get zero movement speed
    /// - Mobs without specified speed default to 0.0
    pub fn new(game_map: &GameMap) -> Self {
        let mut player: Option<Unit> = None;
        let mut mobs: Vec<Unit> = Vec::new();

        for mob in game_map.iter_mobs() {
            if mob.is_player {
                player = Some(Unit {
                    x: mob.x_start as f32,
                    y: mob.y_start as f32,
                    x_speed: 10.,
                    y_speed: 10.,
                });
                continue;
            }

            if let Some(beh) = &mob.behaviour {
                let mob_direction = beh.direction.as_deref().unwrap_or("none");
                let mob_speed = beh.speed.unwrap_or(0.0);

                mobs.push(Unit {
                    x: mob.x_start as f32,
                    y: mob.y_start as f32,
                    x_speed: match mob_direction {
                        "right" => mob_speed,
                        "left" => -mob_speed,
                        _ => 0.0,
                    },
                    y_speed: match mob_direction {
                        "up" => -mob_speed,
                        "down" => mob_speed,
                        _ => 0.0,
                    },
                });
            } else {
                mobs.push(Unit {
                    x: mob.x_start as f32,
                    y: mob.y_start as f32,
                    x_speed: 0.0,
                    y_speed: 0.0,
                });
            }
        }

        Self { player: player.unwrap(), mobs }
    }
}
