use std::vec;

use crate::input::InputSnapshot;

use super::State;

fn abs_vector(vec: (f32, f32)) -> f32 {
    let (dx, dy) = vec;
    return (dx * dx + dy * dy).sqrt();
}

fn normalize_vector(vec: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = vec;
    let distance = abs_vector(vec);
    if distance > 0.1 {
        (dx / distance, dy / distance)
    } else {
        (0., 0.)
    }
}

pub fn make_step(curState: &mut State, inputState: &InputSnapshot) {
    let player_speed = 0.75;
    let collision_distance = 10.0;

    let player = &mut curState.player;

    let mut player_move_vec = (0.0, 0.0);
    player_move_vec.0 += if inputState.right { 1.0 } else { 0.0 };
    player_move_vec.0 += if inputState.left { -1.0 } else { 0.0 };
    player_move_vec.1 += if inputState.up { -1.0 } else { 0.0 };
    player_move_vec.1 += if inputState.down { 1.0 } else { 0.0 };

    let norm = normalize_vector(player_move_vec);
    player.x += norm.0 * player_speed;
    player.y += norm.1 * player_speed;

    // make that mob go to player
    for mob in &mut curState.mobs {
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
