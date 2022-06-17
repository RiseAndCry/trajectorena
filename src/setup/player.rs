use crate::prelude::*;

pub fn spawn_player_1(commands: &mut Commands) {
    let mut position = Vec3::ZERO;
    position.y = -ARENA_SIZE.height_half + CASTLE_WALL_THICKNESS_HALF + PLAYER_SIZE.height_half;

    commands.spawn_bundle(PlayerBundle::new(
        Player::One,
        position,
        Color::BLUE
    ));
}

pub fn spawn_player_2(commands: &mut Commands) {
    let mut position = Vec3::ZERO;
    position.y = ARENA_SIZE.height_half - CASTLE_WALL_THICKNESS_HALF - PLAYER_SIZE.height_half;

    commands.spawn_bundle(PlayerBundle::new(
        Player::Two,
        position,
        Color::RED
    ));
}