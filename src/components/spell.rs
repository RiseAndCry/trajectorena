#![warn(clippy::pedantic)]

use crate::prelude::*;

#[derive(Component)]
pub struct Spell;

#[derive(Bundle)]
pub struct SpellBundle {
    pub spell: Spell,
    pub movement: Movement,
    pub despawnable: Despawnable,
    #[bundle]
    pub sprite: SpriteBundle,
}