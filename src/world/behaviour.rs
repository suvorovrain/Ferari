use crate::input::InputSnapshot;

use super::State;

fn normalize_vector(dx: f32, dy: f32) -> (f32, f32) {
    let distance = (dx * dx + dy * dy).sqrt();
    if distance > 0.1 {
        (dx / distance, dy / distance)
    } else {
        (0., 0.)
    }
}

pub fn make_step(curState: &mut State, inputState: &InputSnapshot) {
    let player = &mut curState.player;
    let player_speed = 0.2;
    player.x += if inputState.right { player_speed } else { 0.0 };
    player.x += if inputState.left { -player_speed } else { 0.0 };
    player.y += if inputState.up { player_speed } else { 0.0 };
    player.y += if inputState.down { -player_speed } else { 0.0 };

    // make that mob go to player
    for mob in &mut curState.mobs {
        let vec_to = (player.x - mob.x, player.y - mob.y);
        let norm = normalize_vector(vec_to.0, vec_to.1);
        // length of vec_move is |speed|
        let mob_speed = (if mob.x_speed != 0. { mob.x_speed } else { mob.y_speed }).abs();
        let vec_move = (norm.0 * mob_speed, norm.1 * mob_speed);

        mob.x += vec_move.0;
        mob.y += vec_move.1;
    }
}
