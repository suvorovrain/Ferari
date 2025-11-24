use crate::world::{Camera, Unit};

use ferari::world::State;

/// Returns a list of game objects that are currently visible within the camera's view.
///
/// This function filters all game units (player and mobs) to only include those
/// that fall within the camera's current field of view. The visibility is determined
/// by the camera's position and viewport dimensions.
///
/// # Arguments
///
/// * `cur_state` - The current game state containing all units
/// * `camera` - The camera that defines the visible area of the game world
///
/// # Returns
///
/// A vector containing all [`Unit`] objects that are currently visible to the camera.
/// The player unit is always included first, followed by any visible mobs.
pub fn get_visible_objects(cur_state: &State, camera: &Camera) -> Vec<Unit> {
    let mut units = Vec::new();
    units.push(cur_state.player.clone());
    units.extend(cur_state.mobs.clone());

    units.into_iter().filter(|mob| camera.is_visible(mob.x, mob.y)).collect()
}

#[cfg(test)]
mod visible_objects_tests {
    use super::*;

    #[derive(Clone)]
    struct DummyUnit {
        x: f32,
        y: f32,
        x_speed: f32,
        y_speed: f32,
    }

    #[derive(Clone)]
    struct DummyState {
        player: DummyUnit,
        mobs: Vec<DummyUnit>,
    }

    impl DummyState {
        fn to_real_state(&self) -> State {
            State {
                player: Unit {
                    x: self.player.x,
                    y: self.player.y,
                    x_speed: self.player.x_speed,
                    y_speed: self.player.y_speed,
                },
                mobs: self
                    .mobs
                    .iter()
                    .map(|m| Unit { x: m.x, y: m.y, x_speed: m.x_speed, y_speed: m.y_speed })
                    .collect(),
            }
        }
    }

    #[test]
    fn test_get_visible_objects_player_included() {
        let dummy_state = DummyState {
            player: DummyUnit { x: 0.0, y: 0.0, x_speed: 0.0, y_speed: 0.0 },
            mobs: vec![],
        };
        let state = dummy_state.to_real_state();

        let camera = Camera::new(0.0, 0.0, 800, 600);
        let visible = get_visible_objects(&state, &camera);

        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].x, state.player.x);
        assert_eq!(visible[0].y, state.player.y);
    }

    #[test]
    fn test_get_visible_objects_mobs_visible() {
        let dummy_state = DummyState {
            player: DummyUnit { x: 0.0, y: 0.0, x_speed: 0.0, y_speed: 0.0 },
            mobs: vec![
                DummyUnit { x: 10.0, y: 10.0, x_speed: 0.0, y_speed: 0.0 },
                DummyUnit { x: 1000.0, y: 1000.0, x_speed: 0.0, y_speed: 0.0 },
            ],
        };
        let state = dummy_state.to_real_state();
        let camera = Camera::new(0.0, 0.0, 50, 50);

        let visible = get_visible_objects(&state, &camera);
        assert_eq!(visible.len(), 2);
        assert_eq!(visible[1].x, 10.0);
        assert_eq!(visible[1].y, 10.0);
    }

    #[test]
    fn test_get_visible_objects_mobs_outside_not_included() {
        let dummy_state = DummyState {
            player: DummyUnit { x: 0.0, y: 0.0, x_speed: 0.0, y_speed: 0.0 },
            mobs: vec![DummyUnit { x: 100.0, y: 100.0, x_speed: 0.0, y_speed: 0.0 }],
        };
        let state = dummy_state.to_real_state();
        let camera = Camera::new(0.0, 0.0, 50, 50);

        let visible = get_visible_objects(&state, &camera);
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].x, state.player.x);
    }

    #[test]
    fn test_get_visible_objects_multiple_mobs() {
        let dummy_state = DummyState {
            player: DummyUnit { x: 0.0, y: 0.0, x_speed: 0.0, y_speed: 0.0 },
            mobs: vec![
                DummyUnit { x: 5.0, y: 5.0, x_speed: 0.0, y_speed: 0.0 },
                DummyUnit { x: 20.0, y: 20.0, x_speed: 0.0, y_speed: 0.0 },
                DummyUnit { x: 100.0, y: 100.0, x_speed: 0.0, y_speed: 0.0 },
            ],
        };
        let state = dummy_state.to_real_state();
        let camera = Camera::new(0.0, 0.0, 50, 50);

        let visible = get_visible_objects(&state, &camera);
        assert_eq!(visible.len(), 3);
        let positions: Vec<_> = visible.iter().map(|u| (u.x, u.y)).collect();
        assert!(positions.contains(&(0.0, 0.0)));
        assert!(positions.contains(&(5.0, 5.0)));
        assert!(positions.contains(&(20.0, 20.0)));
    }
}
