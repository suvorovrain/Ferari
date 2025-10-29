use crate::input::InputSnapshot;

use super::State;

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
        println!("{}", abs_vector(vec_to));
        let norm = normalize_vector(vec_to);
        // length of vec_move is |speed|
        let mob_speed = (if mob.x_speed != 0. { mob.x_speed } else { mob.y_speed }).abs();
        let vec_move = (norm.0 * mob_speed, norm.1 * mob_speed);

        mob.x += vec_move.0;
        mob.y += vec_move.1;
    }
}
