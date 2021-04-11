#![warn(clippy::pedantic)]

mod collision;
mod map_builder;
mod player;
mod spell;

pub use crate::systems::collision::*;
pub use crate::systems::map_builder::*;
pub use crate::systems::player::*;
pub use crate::systems::spell::*;