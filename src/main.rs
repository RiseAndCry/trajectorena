#![warn(clippy::pedantic)]

use bevy::{
    prelude::*,
};

// todo handle resizing, different resolutions
const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;
const SPELL_VELOCITY: f32 = 200.0;

struct Spell {
    velocity: Vec3,
}


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
        .add_system(spell_movement_system.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {

    spawn_arena_bounds(commands, &mut materials);
    spawn_spell(commands, &mut materials);

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

fn spawn_spell(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 1.0)),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Spell {
            velocity: SPELL_VELOCITY * Vec3::new(0.5, 0.5, 0.0).normalize(),
        });
}

fn spell_movement_system(time: Res<Time>, mut spell_query: Query<(&Spell, &mut Transform)>) {
    // limit the maximum distance covered to 2 percent of velocity (each tick)
    let delta_seconds = f32::min(0.02, time.delta_seconds());

    for (spell, mut transform) in spell_query.iter_mut() {
        transform.translation += spell.velocity * delta_seconds;
    }
}