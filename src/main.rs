#![warn(clippy::pedantic)]

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// todo handle resizing, different resolutions
const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;
const SPELL_VELOCITY: f32 = 400.0;

struct Spell {
    velocity: Vec3,
}

#[derive(PartialEq)]
enum Collider {
    Solid,
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
        .add_system(spell_collision_system.system())
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
    })
    .with(Collider::Solid);

    // right
    commands.spawn(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    })
    .with(Collider::Solid);

    // bottom
    commands.spawn(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    })
    .with(Collider::Solid);

    // left
    commands.spawn(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    })
    .with(Collider::Solid);
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

fn spell_collision_system(
    mut spell_query: Query<(&mut Spell, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (mut spell, spell_transform, sprite) in spell_query.iter_mut() {
        let spell_size = sprite.size;
        let spell_velocity = &mut spell.velocity;

        for (_, collider, transform, sprite) in collider_query.iter() {
            // collision is not detected if the object is moving too fast, so either it has to be
            // slower, or one of the colliding objects needs to be bigger in size.
            // https://github.com/bevyengine/bevy/issues/1240
            let collision = collide(
                spell_transform.translation,
                spell_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = spell_velocity.x > 0.0,
                    Collision::Right => reflect_x = spell_velocity.x < 0.0,
                    Collision::Top => reflect_y = spell_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = spell_velocity.y > 0.0,
                }

                if reflect_x {
                    spell_velocity.x = -spell_velocity.x;
                }

                if reflect_y {
                    spell_velocity.y = -spell_velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is also in collision
                if *collider == Collider::Solid {
                    break;
                }
            }
        }
    }
}
