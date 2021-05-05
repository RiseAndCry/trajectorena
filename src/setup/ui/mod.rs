#![warn(clippy::pedantic)]

pub mod main_menu;
pub mod health_text;
pub mod game_over_text;

pub use crate::setup::ui::main_menu::*;
pub use crate::setup::ui::health_text::*;
pub use crate::setup::ui::game_over_text::*;

