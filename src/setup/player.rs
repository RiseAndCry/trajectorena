#![warn(clippy::pedantic)]

use crate::prelude::*;

const PLAYER_STARTING_TRANSLATION: (f32, f32, f32) =
    (0.0, -SCREEN_HEIGHT / 2.0 + ARENA_WALL_THICKNESS + 10.0, 0.0);

pub fn spawn_player(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let player_material = materials.add(Color::YELLOW.into());

    commands
        .spawn_bundle(SpriteBundle {
            material: player_material.clone(),
            transform: Transform::from_translation(Vec3::from(PLAYER_STARTING_TRANSLATION)),
            sprite: Sprite::new(Vec2::from(PLAYER_SIZE)),
            ..Default::default()
        })
        .insert(Player::new());
}