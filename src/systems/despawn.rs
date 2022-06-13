use crate::prelude::*;

pub fn despawn_system(
    mut commands: Commands,
    entity_query: Query<Entity>,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}