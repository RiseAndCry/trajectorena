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
    mut castle_health: ResMut<CastleHealth>
) {
    for (entity, transform, sprite, _) in transformable_query.iter_mut() {
        let sprite_size = sprite.custom_size.expect("Sprite size must be set");
        let despawn_top_y = SCREEN_HEIGHT / 2.0 - CASTLE_WALL_THICKNESS + sprite_size.y / 2.0;
        let despawn_bottom_y = -SCREEN_HEIGHT / 2.0 + CASTLE_WALL_THICKNESS - sprite_size.y / 2.0;

        if transform.translation.y >= despawn_top_y || transform.translation.y <= despawn_bottom_y
        {
            commands.entity(entity).despawn();
            castle_health.health -= 1;
        }
    }
}

