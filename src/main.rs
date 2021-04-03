#![warn(clippy::pedantic)]

use bevy::prelude::*;

// todo handle resizing, different resolutions
const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Trajectorena".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {

    spawn_arena_bounds(commands, &mut materials);

    commands
        .spawn(Camera2dBundle::default());

}

fn spawn_arena_bounds(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 5.0;
    let bounds = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT);

    // top
    commands.spawn(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });

    // right
    commands.spawn(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });

    // bottom
    commands.spawn(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });

    // left
    commands.spawn(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });
}