#![warn(clippy::pedantic)]

use crate::prelude::*;

pub const ARENA_SIZE: (f32, f32) = (SCREEN_WIDTH / 2.0, SCREEN_HEIGHT);
pub const ARENA_WALL_THICKNESS: f32 = 5.0;

pub fn spawn_arena_bounds(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let arena = Vec2::from(ARENA_SIZE);

    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, arena.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(
                arena.x + ARENA_WALL_THICKNESS,
                ARENA_WALL_THICKNESS,
            )),
            ..Default::default()
        })
        .insert(Collider::Solid);

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

    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -arena.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(
                arena.x + ARENA_WALL_THICKNESS,
                ARENA_WALL_THICKNESS,
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