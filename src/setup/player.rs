use crate::prelude::*;

pub const PLAYER_VELOCITY: f32 = 1200.0;
const PLAYER_STARTING_TRANSLATION: (f32, f32, f32) =
    (0.0, -ARENA_SIZE.height_half + CASTLE_WALL_THICKNESS_HALF + PLAYER_SIZE.height_half, 0.0);

pub fn spawn_player_1(commands: &mut Commands) {
    let player_starting_position = Vec3::from(PLAYER_STARTING_TRANSLATION);

    commands.spawn_bundle(PlayerBundle {
        player: Player::One,
        sprite: SpriteBundle {
            transform: Transform::from_translation(player_starting_position),
            sprite: Sprite {
                custom_size: Some(Vec2::from(PLAYER_SIZE)),
                color: Color::BLUE,
                ..default()
            },
            ..default()
        },
        movement: Movement::new(Vec3::from((PLAYER_VELOCITY, PLAYER_VELOCITY, 0.0))),
    });
}