#![warn(clippy::pedantic)]

use crate::prelude::*;

pub const ARENA_SIZE: (f32, f32, f32) = (SCREEN_WIDTH / 2.0, SCREEN_HEIGHT, 0.0);
pub const ARENA_WALL_THICKNESS: f32 = 5.0;
pub const CASTLE_WALL_THICKNESS: f32 = 20.0;

const CASTLE_WALL_LENGTH: f32 = ARENA_SIZE.0 / 4.0;
const CASTLE_WALL_Y_TRANSLATION: f32 = ARENA_SIZE.1 / 3.0;

pub fn spawn_arena_bounds(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::WHITE.into());
    let arena = Vec3::from(ARENA_SIZE);

    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(arena.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(
                ARENA_WALL_THICKNESS,
                arena.y + ARENA_WALL_THICKNESS,
            )),
            ..Default::default()
        })
        .insert(Collider::Solid);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-arena.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(
                ARENA_WALL_THICKNESS,
                arena.y + ARENA_WALL_THICKNESS,
            )),
            ..Default::default()
        })
        .insert(Collider::Solid);
}

pub fn spawn_castles(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let castle_material = materials.add(Color::GREEN.into());
    let arena = Vec3::from(ARENA_SIZE);

    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: castle_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, arena.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(
                arena.x,
                CASTLE_WALL_THICKNESS,
            )),
            ..Default::default()
        });

    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: castle_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -arena.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(
                arena.x,
                CASTLE_WALL_THICKNESS,
            )),
            ..Default::default()
        });
}

pub fn spawn_castle_walls(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let arena = Vec3::from(ARENA_SIZE);

    let sprite = Sprite::new(Vec2::new(
        CASTLE_WALL_LENGTH,
        ARENA_WALL_THICKNESS,
    ));

    // player 1 castle wall left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                -arena.x / 2.0 + CASTLE_WALL_LENGTH / 2.0,
                -CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite: sprite.clone(),
            ..Default::default()
        })
        .insert(Collider::Solid);

    // player 1 castle wall right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                arena.x / 2.0 - CASTLE_WALL_LENGTH / 2.0,
                -CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite: sprite.clone(),
            ..Default::default()
        })
        .insert(Collider::Solid);

    // player 2 castle wall left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                -arena.x / 2.0 + CASTLE_WALL_LENGTH / 2.0,
                CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite: sprite.clone(),
            ..Default::default()
        })
        .insert(Collider::Solid);

    // player 2 castle wall right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                arena.x / 2.0 - CASTLE_WALL_LENGTH / 2.0,
                CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite: sprite.clone(),
            ..Default::default()
        })
        .insert(Collider::Solid);
}