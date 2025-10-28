use crate::world::{Camera, Unit};

use super::State;

pub fn get_visible_objects(cur_state: &State, camera: &Camera) -> Vec<Unit> {
    let mut units = Vec::new();
    units.push(cur_state.player.clone());
    units.extend(cur_state.mobs.clone());

    units.into_iter().filter(|mob| camera.is_visible(mob.x, mob.y)).collect()
}
