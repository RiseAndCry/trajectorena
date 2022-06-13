use crate::prelude::*;

pub fn health_update_system(
    mut query: Query<&mut Text, With<HealthText>>,
    castle_health: ResMut<CastleHealth>
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = castle_health.health.to_string();
    }
}