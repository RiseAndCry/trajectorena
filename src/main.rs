#![warn(clippy::pedantic)]

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// todo handle resizing, different resolutions
const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

const ARENA_SIZE: (f32, f32) = (SCREEN_WIDTH / 2.0, SCREEN_HEIGHT);
const ARENA_WALL_THICKNESS: f32 = 5.0;

const SPELL_COOLDOWN: f32 = 0.5;
const SPELL_VELOCITY: f32 = 400.0;
const SPELL_STARTING_POSITION_OFFSET: (f32, f32, f32) = (0.0, 20.0, 0.0);

const PLAYER_SIZE: (f32, f32) = (16.0, 16.0);
const PLAYER_SPEED: f32 = 300.0;
const PLAYER_STARTING_TRANSLATION: (f32, f32, f32) =
    (0.0, -SCREEN_HEIGHT / 2.0 + ARENA_WALL_THICKNESS + 10.0, 0.0);


struct Player {
    speed: f32,
}

impl Player {
    fn new() -> Self {
        Player {
            speed: PLAYER_SPEED,
        }
    }
}

struct SpellCooldown {
    timer: Timer,
}

impl SpellCooldown {
    fn new() -> Self {
        SpellCooldown {
            timer: Timer::from_seconds(SPELL_COOLDOWN, false),
        }
    }
}

struct Spell;

struct Movement {
    velocity: Vec3,
}

// todo add default trait
impl Movement {
    fn new(velocity: Vec3) -> Self {
        Movement {
            velocity
        }
    }
}

#[derive(Bundle)]
struct SpellBundle {
    spell: Spell,
    movement: Movement,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(PartialEq)]
enum Collider {
    Solid,
}

// todo separate code into modules
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Trajectorena".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .insert_resource(SpellCooldown::new())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(spell_movement_system.system())
        .add_system(spell_collision_system.system())
        .add_system(player_movement_system.system())
        .add_system(player_shooting_system.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    spawn_arena_bounds(&mut commands, &mut materials);
    spawn_player(&mut commands, &mut materials);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_arena_bounds(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
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

fn spawn_spell(mut commands: Commands, player_transform: &Transform) {

    let mut spell_starting_position = player_transform.translation.clone();
    spell_starting_position += Vec3::from(SPELL_STARTING_POSITION_OFFSET);

    commands.spawn_bundle(SpellBundle {
        spell: Spell,
        sprite: SpriteBundle {
            transform: Transform::from_translation(spell_starting_position),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        },
        movement: Movement::new(SPELL_VELOCITY * Vec3::new(0.5, 0.5, 0.0).normalize()),
    });
}

fn spawn_player(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
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

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let arena_size = Vec2::from(ARENA_SIZE);
        let mut movement = Vec2::new(0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            movement.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            movement.y -= 1.0;
        }

        let translation = &mut transform.translation;
        translation.x += time.delta_seconds() * movement.x * player.speed;
        translation.y += time.delta_seconds() * movement.y * player.speed;

        // bound the player within the walls
        let player_one_side_size = Vec2::from(PLAYER_SIZE).x / 2.0;

        translation.x = translation
            .x
            .min(arena_size.x / 2.0 - ARENA_WALL_THICKNESS - player_one_side_size)
            .max(-arena_size.x / 2.0 + ARENA_WALL_THICKNESS + player_one_side_size);

        translation.y = translation
            .y
            .min(arena_size.y / 2.0 - ARENA_WALL_THICKNESS - player_one_side_size)
            .max(-arena_size.y / 2.0 + ARENA_WALL_THICKNESS + player_one_side_size);
    }
}

// todo shoot in certain direction
fn player_shooting_system(
    time: Res<Time>,
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut spell_cooldown: ResMut<SpellCooldown>,
    player_query: Query<(&Player, &Transform)>,
) {
    // todo spell_cooldown should be replaced with SpellBundle timer
    spell_cooldown.timer.tick(time.delta());
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }
    if !spell_cooldown.timer.finished() {
        return;
    }

    let (_, player_transform) = player_query.single()
        .expect("Second player is not implemented yet");

    spawn_spell(commands, player_transform);
    spell_cooldown.timer.reset();
}

fn spell_movement_system(
    time: Res<Time>,
    mut spell_query: Query<(&Spell, &Movement, &mut Transform)>
) {
    // limit the maximum distance covered to 2 percent of velocity (each tick)
    let delta_seconds = f32::min(0.02, time.delta_seconds());

    for (_, movement, mut transform) in spell_query.iter_mut() {
        transform.translation += movement.velocity * delta_seconds;
    }
}

fn spell_collision_system(
    mut spell_query: Query<(&Spell, &Transform, &mut Movement, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (_, spell_transform, mut spell_movement, sprite) in spell_query.iter_mut() {
        let spell_size = sprite.size;

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
                    Collision::Left => reflect_x = spell_movement.velocity.x > 0.0,
                    Collision::Right => reflect_x = spell_movement.velocity.x < 0.0,
                    Collision::Top => reflect_y = spell_movement.velocity.y < 0.0,
                    Collision::Bottom => reflect_y = spell_movement.velocity.y > 0.0,
                }

                if reflect_x {
                    spell_movement.velocity.x = -spell_movement.velocity.x;
                }

                if reflect_y {
                    spell_movement.velocity.y = -spell_movement.velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is also in collision
                if *collider == Collider::Solid {
                    break;
                }
            }
        }
    }
}
