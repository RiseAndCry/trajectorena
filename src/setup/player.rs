use crate::prelude::*;

const PLAYER_STARTING_TRANSLATION: (f32, f32, f32) =
    (0.0, -SCREEN_HEIGHT / 2.0 + ARENA_WALL_THICKNESS + 10.0, 0.0);

pub fn spawn_player(commands: &mut Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::from(PLAYER_STARTING_TRANSLATION)),
            sprite: Sprite {
                custom_size: Some(Vec2::from(PLAYER_SIZE)),
                color: Color::YELLOW,
                ..default()
            },
            ..default()
        })
        .insert(Player::new());
}