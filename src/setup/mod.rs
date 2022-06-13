pub mod ui;

pub use crate::setup::ui::main_menu::*;
pub use crate::setup::ui::health_text::*;
pub use crate::setup::ui::game_over_text::*;

mod map_builder;
mod player;
mod spell;

pub use crate::setup::map_builder::*;
pub use crate::setup::player::*;
pub use crate::setup::spell::*;
