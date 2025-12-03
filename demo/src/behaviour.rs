use crate::input::InputSnapshot;

use ferari::world::State;

/// Calculates the absolute value (length) of a 2D vector.
///
/// # Arguments
/// * `vec` - A tuple representing a 2D vector (x, y)
///
/// # Returns
/// * The length of the vector as f32
fn abs_vector(vec: (f32, f32)) -> f32 {
    let (dx, dy) = vec;
    (dx * dx + dy * dy).sqrt()
}

/// Normalizes a 2D vector to unit length.
///
/// If the vector's length is not more than 0.1, returns a zero vector
/// to avoid division by very small numbers.
///
/// # Arguments
/// * `vec` - A tuple representing a 2D vector (x, y)
///
/// # Returns
/// * A normalized vector as tuple (x, y) or zero vector if length is small
fn normalize_vector(vec: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = vec;
    let distance = abs_vector(vec);
    if distance > 0.1 {
        (dx / distance, dy / distance)
    } else {
        (0., 0.)
    }
}

/// Updates the game state for one simulation step.
///
/// Handles player movement based on input and mob behaviour.
///
/// # Arguments
/// * `curr_state` - Mutable reference to the current game state
/// * `input_state` - Reference to the current input snapshot
pub fn make_step(curr_state: &mut State, input_state: &InputSnapshot) {
    let player_speed = 0.75;
    let collision_distance = 10.0;

    let player = &mut curr_state.player;

    let mut player_move_vec = (0.0, 0.0);
    player_move_vec.0 += if input_state.right { 1.0 } else { 0.0 };
    player_move_vec.0 += if input_state.left { -1.0 } else { 0.0 };
    player_move_vec.1 += if input_state.up { -1.0 } else { 0.0 };
    player_move_vec.1 += if input_state.down { 1.0 } else { 0.0 };

    let norm = normalize_vector(player_move_vec);
    player.x += norm.0 * player_speed;
    player.y += norm.1 * player_speed;

    // make that mob go to player
    for mob in &mut curr_state.mobs {
        let vec_to = (player.x - mob.x, player.y - mob.y);
        if abs_vector(vec_to) <= collision_distance {
            let vec_from = (mob.x - player.x, mob.y - player.y);
            let norm = normalize_vector(vec_from);
            mob.x = player.x + norm.0 * collision_distance;
            mob.y = player.y + norm.1 * collision_distance;
            continue;
        }
        let norm = normalize_vector(vec_to);
        // length of vec_move is |speed|
        let mob_speed = (if mob.x_speed != 0. { mob.x_speed } else { mob.y_speed }).abs();
        let vec_move = (norm.0 * mob_speed, norm.1 * mob_speed);

        mob.x += vec_move.0;
        mob.y += vec_move.1;
    }
}

#[cfg(test)]
mod tests {
    use super::State;
    use crate::assets::GameMap;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_abs_vector_zero() {
        assert_eq!(abs_vector((0.0, 0.0)), 0.0);
    }

    #[test]
    fn test_abs_vector_nonzero() {
        let len = abs_vector((3.0, 4.0));
        assert!((len - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_normalize_vector_basic() {
        let n = normalize_vector((3.0, 4.0));
        assert!(((n.0 * n.0 + n.1 * n.1).sqrt() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_normalize_vector_small_vector_returns_zero() {
        let n = normalize_vector((0.01, 0.01));
        assert_eq!(n, (0.0, 0.0));
    }

    fn make_test_state() -> State {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_root = manifest_dir.join("..");

        let map_path = project_root.join("examples/input.json");
        let game_map = GameMap::load(map_path).expect("failed to load game map for tests");

        let mut state = State::new(&game_map);

        state.player.x = 0.0;
        state.player.y = 0.0;
        state.player.x_speed = 0.0;
        state.player.y_speed = 0.0;

        if state.mobs.is_empty() {
            state.mobs.push(crate::world::Unit {
                x: 100.0,
                y: 0.0,
                x_speed: -0.5,
                y_speed: 0.0,
                ..Default::default()
            });
        }

        state
    }

    #[test]
    fn test_player_moves_right() {
        let mut state = make_test_state();

        let input = crate::input::InputSnapshot {
            up: false,
            down: false,
            left: false,
            right: true,
            escape: false,
        };

        make_step(&mut state, &input);

        assert!((state.player.x - 0.75).abs() < 1e-5);
        assert!((state.player.y - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_player_moves_up_left_diagonal() {
        let mut state = make_test_state();

        let input = crate::input::InputSnapshot {
            up: true,
            down: false,
            left: true,
            right: false,
            escape: false,
        };

        make_step(&mut state, &input);

        let dx = state.player.x;
        let dy = state.player.y;
        let len = (dx * dx + dy * dy).sqrt();
        assert!((len - 0.75).abs() < 1e-5);
        assert!(dx < 0.0 && dy < 0.0);
    }

    #[test]
    fn test_mob_moves_toward_player() {
        let mut state = make_test_state();
        state.mobs[0].x = 50.0;
        state.mobs[0].y = 0.0;
        state.mobs[0].x_speed = -0.5;
        state.mobs[0].y_speed = 0.0;

        let input = crate::input::InputSnapshot {
            up: false,
            down: false,
            left: false,
            right: false,
            escape: false,
        };

        make_step(&mut state, &input);

        assert!(state.mobs[0].x < 50.0);
        assert!(state.mobs[0].y.abs() < 1e-3);
    }

    #[test]
    fn test_collision_pushes_mob_back() {
        let mut state = make_test_state();

        state.mobs[0].x = 2.0;
        state.mobs[0].y = 0.0;

        let input = crate::input::InputSnapshot {
            up: false,
            down: false,
            left: false,
            right: false,
            escape: false,
        };

        make_step(&mut state, &input);

        let vec_from = (state.mobs[0].x - state.player.x, state.mobs[0].y - state.player.y);
        let dist = (vec_from.0 * vec_from.0 + vec_from.1 * vec_from.1).sqrt();
        assert!((dist - 10.0).abs() < 1e-3);
    }
}
