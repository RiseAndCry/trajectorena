#![warn(clippy::pedantic)]

pub mod ui;

pub use crate::components::ui::health_text::*;
pub use crate::components::ui::game_over_text::*;
pub use crate::components::ui::button_materials::*;

mod movement;
mod player;
mod spell;
mod collision;
mod despawn;
mod app_state;

pub use crate::components::movement::*;
pub use crate::components::player::*;
pub use crate::components::spell::*;
pub use crate::components::collision::*;
pub use crate::components::despawn::*;
pub use crate::components::app_state::*;