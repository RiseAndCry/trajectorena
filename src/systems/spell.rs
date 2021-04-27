#![warn(clippy::pedantic)]

use crate::prelude::*;

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

