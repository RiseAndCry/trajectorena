#![warn(clippy::pedantic)]

use crate::prelude::*;

pub const SPELL_SIZE: (f32, f32) = (20.0, 20.0);

const SPELL_VELOCITY: f32 = 400.0;
const SPELL_STARTING_POSITION_OFFSET: (f32, f32, f32) = (0.0, 20.0, 0.0);

pub fn spawn_spell(commands: &mut Commands, player_transform: &Transform, direction: Vec3) {
    let mut spell_starting_position = player_transform.translation.clone();
    spell_starting_position += Vec3::from(SPELL_STARTING_POSITION_OFFSET);

    commands.spawn_bundle(SpellBundle {
        spell: Spell,
        sprite: SpriteBundle {
            transform: Transform::from_translation(spell_starting_position),
            sprite: Sprite::new(Vec2::from(SPELL_SIZE)),
            ..Default::default()
        },
        movement: Movement::new(SPELL_VELOCITY * direction.normalize()),
        despawnable: Despawnable,
    });
}