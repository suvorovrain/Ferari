mod atlas;
mod gamemap;

pub use atlas::{Atlas, Frame};
pub use gamemap::{GameMap, Object, Tile};

#[cfg(test)]
pub use gamemap::{Behaviour, BehaviourType, Mob};
