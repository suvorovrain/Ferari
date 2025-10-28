use crate::input::InputSnapshot;

use super::State;

pub fn make_step(curState: &mut State, inputState: &InputSnapshot) {
    let player = &mut curState.player;
    let player_speed = 0.12;
    player.x += if inputState.right { player_speed } else { 0.0 };
    player.x += if inputState.left { -player_speed } else { 0.0 };
    player.y += if inputState.up { player_speed } else { 0.0 };
    player.y += if inputState.down { -player_speed } else { 0.0 };

    for mob in &mut curState.mobs {
        mob.x += mob.x_speed;
        mob.y += mob.y_speed;
    }
}
