#![warn(clippy::pedantic)]

mod movement;
mod player;
mod spell;
mod collision;

pub use crate::components::movement::*;
pub use crate::components::player::*;
pub use crate::components::spell::*;
pub use crate::components::collision::*;