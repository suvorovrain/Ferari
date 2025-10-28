use crate::world::{Camera, Unit};

use super::State;

pub fn get_visible_objects(cur_state: &State, camera: &Camera) -> Vec<Unit> {
    cur_state
        .mobs
        .iter()
        .filter(|mob| camera.is_visible(mob.x, mob.y))
        .cloned() // клонируем каждый элемент
        .collect()
}
