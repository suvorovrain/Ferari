use crate::assets::GameMap;

#[derive(Debug)]
pub struct State {
    pub player: Unit,
    pub mobs: Vec<Unit>,
}
#[derive(Debug,Clone)]
pub struct Unit {
    pub x: f32,
    pub y: f32,
    pub x_speed: f32,
    pub y_speed: f32,
}

impl Unit {
    /// Creates a new `Unit` with given `x` and `y` coordinates.
    ///
    /// # Returns
    ///
    /// A new `Unit` instance with given `x` and `y` coordinates.
    pub fn new(x: f32, y: f32, x_speed: f32, y_speed: f32) -> Self {
        Self { x: x, y: y, x_speed: x_speed, y_speed: y_speed }
    }
}

impl State {
    /// Creates a new `State` with given player and mobs.
    ///
    /// # Returns
    ///
    /// A new `State` instance with given player and mobs.
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

        Self { player: player.unwrap(), mobs: mobs }
    }
}
