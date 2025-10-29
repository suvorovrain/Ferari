use crate::world::{Camera, Unit};

use super::State;

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
