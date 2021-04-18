#![warn(clippy::pedantic)]

mod map_builder;
mod ui_builder;
mod collision;
mod player;
mod spell;
mod despawn;

pub use crate::systems::map_builder::*;
pub use crate::systems::ui_builder::*;
pub use crate::systems::collision::*;
pub use crate::systems::player::*;
pub use crate::systems::spell::*;
pub use crate::systems::despawn::*;