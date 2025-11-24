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

#[cfg(test)]
mod state_tests {
    use crate::assets::{Behaviour, BehaviourType, GameMap, Mob};
    use crate::world::State;

    fn make_test_map() -> GameMap {
        let mut mobs = std::collections::HashMap::new();

        mobs.insert(
            "player".to_string(),
            Mob {
                name: "player".to_string(),
                x_start: 0,
                y_start: 0,
                asset: "knight".to_string(),
                is_player: true,
                behaviour: None,
            },
        );

        mobs.insert(
            "mob_right".to_string(),
            Mob {
                name: "mob_right".to_string(),
                x_start: 10,
                y_start: 0,
                asset: "imp".to_string(),
                is_player: false,
                behaviour: Some(Behaviour {
                    behaviour_type: BehaviourType::Walker,
                    direction: Some("right".to_string()),
                    speed: Some(1.0),
                }),
            },
        );

        mobs.insert(
            "mob_up".to_string(),
            Mob {
                name: "mob_up".to_string(),
                x_start: 0,
                y_start: 10,
                asset: "ghost".to_string(),
                is_player: false,
                behaviour: Some(Behaviour {
                    behaviour_type: BehaviourType::Walker,
                    direction: Some("up".to_string()),
                    speed: Some(0.5),
                }),
            },
        );

        GameMap {
            name: "test_map".to_string(),
            tile_size: 16,
            size: [5, 5],
            mobs,
            objects: std::collections::HashMap::new(),
            tiles: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_state_new_creates_player_and_mobs() {
        let map = make_test_map();
        let state = State::new(&map);

        assert_eq!(state.player.x, 0.0);
        assert_eq!(state.player.y, 0.0);
        assert_eq!(state.player.x_speed, 10.0);
        assert_eq!(state.player.y_speed, 10.0);

        assert_eq!(state.mobs.len(), 2);

        let mob_right = state.mobs.iter().find(|m| m.x_speed > 0.0).unwrap();
        assert_eq!(mob_right.x_speed, 1.0);
        assert_eq!(mob_right.y_speed, 0.0);
        assert_eq!(mob_right.x, 10.0);
        assert_eq!(mob_right.y, 0.0);

        let mob_up = state.mobs.iter().find(|m| m.y_speed < 0.0).unwrap();
        assert_eq!(mob_up.x_speed, 0.0);
        assert_eq!(mob_up.y_speed, -0.5);
        assert_eq!(mob_up.x, 0.0);
        assert_eq!(mob_up.y, 10.0);
    }

    #[test]
    fn test_state_with_no_mobs_other_than_player() {
        let mut map = make_test_map();
        map.mobs.retain(|_, mob| mob.is_player);
        let state = State::new(&map);

        assert_eq!(state.player.x, 0.0);
        assert_eq!(state.player.y, 0.0);
        assert!(state.mobs.is_empty());
    }

    #[test]
    fn test_mob_with_unknown_or_none_behaviour_defaults_to_zero_speed() {
        let mut mobs = std::collections::HashMap::new();
        mobs.insert(
            "player".to_string(),
            Mob {
                name: "player".to_string(),
                x_start: 0,
                y_start: 0,
                asset: "knight".to_string(),
                is_player: true,
                behaviour: None,
            },
        );
        mobs.insert(
            "mob_none".to_string(),
            Mob {
                name: "mob_none".to_string(),
                x_start: 5,
                y_start: 5,
                asset: "dummy".to_string(),
                is_player: false,
                behaviour: None,
            },
        );
        mobs.insert(
            "mob_unknown".to_string(),
            Mob {
                name: "mob_unknown".to_string(),
                x_start: 10,
                y_start: 10,
                asset: "dummy".to_string(),
                is_player: false,
                behaviour: Some(Behaviour {
                    behaviour_type: BehaviourType::Unknown,
                    direction: Some("left".to_string()),
                    speed: Some(2.0),
                }),
            },
        );

        let map = GameMap {
            name: "test_map".to_string(),
            tile_size: 16,
            size: [5, 5],
            mobs,
            objects: std::collections::HashMap::new(),
            tiles: std::collections::HashMap::new(),
        };

        let state = State::new(&map);
        assert_eq!(state.mobs.len(), 2);

        let mob_none = state.mobs.iter().find(|m| m.x == 5.0).unwrap();
        assert_eq!(mob_none.x_speed, 0.0);
        assert_eq!(mob_none.y_speed, 0.0);

        let mob_unknown = state.mobs.iter().find(|m| m.x == 10.0).unwrap();
        assert_eq!(mob_unknown.x_speed, -2.0);
        assert_eq!(mob_unknown.y_speed, 0.0);
    }

    #[test]
    fn test_player_position_does_not_change_from_map() {
        let map = make_test_map();
        let state = State::new(&map);

        let player_map = map.get_mob("player").unwrap();
        assert_eq!(state.player.x, player_map.x_start as f32);
        assert_eq!(state.player.y, player_map.y_start as f32);
    }
}
