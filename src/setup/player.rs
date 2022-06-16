use crate::prelude::*;

pub const PLAYER_VELOCITY: f32 = 1200.0;
const PLAYER_1_STARTING_TRANSLATION: (f32, f32, f32) =
    (0.0, -ARENA_SIZE.height_half + CASTLE_WALL_THICKNESS_HALF + PLAYER_SIZE.height_half, 0.0);
const PLAYER_2_STARTING_TRANSLATION: (f32, f32, f32) =
    (0.0, ARENA_SIZE.height_half - CASTLE_WALL_THICKNESS_HALF - PLAYER_SIZE.height_half, 0.0);

pub fn spawn_player_1(commands: &mut Commands) {
    spawn_player(commands, Player::One, Vec3::from(PLAYER_1_STARTING_TRANSLATION), Color::BLUE);
}

pub fn spawn_player_2(commands: &mut Commands) {
    spawn_player(commands, Player::Two, Vec3::from(PLAYER_2_STARTING_TRANSLATION), Color::RED);
}

fn spawn_player(commands: &mut Commands, player: Player, starting_position: Vec3, color: Color) {
    commands.spawn_bundle(PlayerBundle {
        player,
        sprite: SpriteBundle {
            transform: Transform::from_translation(starting_position),
            sprite: Sprite {
                custom_size: Some(Vec2::from(PLAYER_SIZE)),
                color,
                ..default()
            },
            ..default()
        },
        movement: Movement::new(Vec3::from((PLAYER_VELOCITY, PLAYER_VELOCITY, 0.0))),
    });
}