#![warn(clippy::pedantic)]

pub mod ui;

pub use crate::systems::ui::main_menu::*;
pub use crate::systems::ui::health_text::*;

mod collision;
mod player;
mod spell;
mod despawn;
mod app_state;

pub use crate::systems::app_state::*;
pub use crate::systems::collision::*;
pub use crate::systems::despawn::*;
pub use crate::systems::player::*;
pub use crate::systems::spell::*;
