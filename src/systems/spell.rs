#![warn(clippy::pedantic)]

use crate::prelude::*;

const SPELL_VELOCITY: f32 = 400.0;
const SPELL_STARTING_POSITION_OFFSET: (f32, f32, f32) = (0.0, 20.0, 0.0);

pub fn spawn_spell(commands: &mut Commands, player_transform: &Transform, direction: Vec3) {

    let mut spell_starting_position = player_transform.translation.clone();
    spell_starting_position += Vec3::from(SPELL_STARTING_POSITION_OFFSET);

    commands.spawn_bundle(SpellBundle {
        spell: Spell,
        sprite: SpriteBundle {
            transform: Transform::from_translation(spell_starting_position),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        },
        movement: Movement::new(SPELL_VELOCITY * direction.normalize()),
    });
}

pub fn spell_movement_system(
    time: Res<Time>,
    mut spell_query: Query<(&Spell, &Movement, &mut Transform)>
) {
    // limit the maximum distance covered to 2 percent of velocity (each tick)
    let delta_seconds = f32::min(0.02, time.delta_seconds());

    for (_, movement, mut transform) in spell_query.iter_mut() {
        transform.translation += movement.velocity * delta_seconds;
    }
}

