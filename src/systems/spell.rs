use crate::prelude::*;

pub fn spell_movement_system(
    mut spell_query: Query<(&Spell, &Movement, &mut Transform)>
) {
    for (_, movement, mut transform) in spell_query.iter_mut() {
        transform.translation += movement.velocity * MOVEMENT_TIME_STEP;
    }
}

pub fn spell_despawn_system(
    mut commands: Commands,
    mut transformable_query: Query<(Entity, &Transform, &Sprite, &Despawnable)>,
    mut spell_out_of_bounds: EventWriter<SpellOutOfBounds>,
) {
    for (entity, transform, sprite, _) in transformable_query.iter_mut() {
        let sprite_size = sprite.custom_size.expect("Sprite size must be set");
        let despawn_top_y = SCREEN_SIZE.height_half - CASTLE_WALL_THICKNESS + sprite_size.y / 2.0;
        let despawn_bottom_y = -SCREEN_SIZE.height_half + CASTLE_WALL_THICKNESS - sprite_size.y / 2.0;

        if transform.translation.y >= despawn_top_y {
            commands.entity(entity).despawn();
            spell_out_of_bounds.send(SpellOutOfBounds::Top);
        }
        if transform.translation.y <= despawn_bottom_y {
            commands.entity(entity).despawn();
            spell_out_of_bounds.send(SpellOutOfBounds::Bottom);
        }
    }
}

pub fn reduce_player_health_when_spell_goes_out_of_bounds(
    mut spell_out_of_bounds: EventReader<SpellOutOfBounds>,
    mut health_query: Query<(&Player, &mut Health)>,
    mut health_text_query: Query<(&HealthText, &mut Text)>,
) {
    for ev in spell_out_of_bounds.iter() {
        let player_to_reduce_health_for = match ev {
            SpellOutOfBounds::Bottom => Player::One,
            SpellOutOfBounds::Top => Player::Two,
        };

        // reduce health
        let mut final_health = 0;
        for (player, mut health) in health_query.iter_mut() {
            if *player == player_to_reduce_health_for {
                health.value -= 1;
                final_health = health.value;
            }
        }

        // change health text value
        for (health_text, mut text) in health_text_query.iter_mut() {
            if health_text.player == player_to_reduce_health_for {
                text.sections[1].value = final_health.to_string();
            }
        }
    }
}

