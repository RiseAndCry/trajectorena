#![warn(clippy::pedantic)]

use crate::prelude::*;

const SPELL_COOLDOWN: f32 = 0.5;

pub struct Spell;

pub struct SpellCooldown {
    pub timer: Timer,
}

impl SpellCooldown {
    pub fn new() -> Self {
        SpellCooldown {
            timer: Timer::from_seconds(SPELL_COOLDOWN, false),
        }
    }
}

#[derive(Bundle)]
pub struct SpellBundle {
    pub spell: Spell,
    pub movement: Movement,
    #[bundle]
    pub sprite: SpriteBundle,
}