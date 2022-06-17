use crate::prelude::*;

pub struct ArenaSize {
    pub width: f32,
    pub width_half: f32,
    pub height: f32,
    pub height_half: f32,
    pub depth: f32,
}

pub const ARENA_SIZE: ArenaSize = ArenaSize {
    width: SCREEN_SIZE.width_half,
    width_half: SCREEN_SIZE.width_half / 2.0,
    height: SCREEN_SIZE.height,
    height_half: SCREEN_SIZE.height_half,
    depth: 0.0,
};

pub const ARENA_WALL_THICKNESS: f32 = 5.0;
pub const ARENA_WALL_THICKNESS_HALF: f32 = ARENA_WALL_THICKNESS / 2.0;
pub const CASTLE_WALL_THICKNESS: f32 = 20.0;
pub const CASTLE_WALL_THICKNESS_HALF: f32 = CASTLE_WALL_THICKNESS / 2.0;

const CASTLE_WALL_LENGTH: f32 = ARENA_SIZE.width / 4.0;
const CASTLE_WALL_LENGTH_HALF: f32 = CASTLE_WALL_LENGTH / 2.0;
pub const CASTLE_WALL_Y_TRANSLATION: f32 = ARENA_SIZE.height / 3.0;

pub fn spawn_arena_bounds(commands: &mut Commands) {
    let sprite = Sprite {
        custom_size: Some(Vec2::new(
            ARENA_WALL_THICKNESS,
            ARENA_SIZE.height + ARENA_WALL_THICKNESS,
        )),
        color: Color::WHITE,
        ..default()
    };

    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                ARENA_SIZE.width_half + ARENA_WALL_THICKNESS_HALF,
                0.0,
                0.0
            )),
            sprite: sprite.clone(),
            ..default()
        })
        .insert(Collider::Solid);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                -ARENA_SIZE.width_half - ARENA_WALL_THICKNESS_HALF,
                0.0,
                0.0
            )),
            sprite,
            ..default()
        })
        .insert(Collider::Solid);
}

pub fn spawn_castles(commands: &mut Commands) {
    let sprite = Sprite {
        custom_size: Some(Vec2::new(
            ARENA_SIZE.width,
            CASTLE_WALL_THICKNESS,
        )),
        color: Color::GREEN,
        ..default()
    };

    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, ARENA_SIZE.height_half, 0.0)),
            sprite: sprite.clone(),
            ..default()
        });

    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -ARENA_SIZE.height_half, 0.0)),
            sprite,
            ..default()
        });
}

pub fn spawn_castle_walls(
    commands: &mut Commands,
) {
    let sprite = Sprite {
        custom_size: Some(Vec2::new(
            CASTLE_WALL_LENGTH,
            ARENA_WALL_THICKNESS,
        )),
        color: Color::rgb(0.8, 0.8, 0.8),
        ..default()
    };

    // player 1 castle wall left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(
                -ARENA_SIZE.width_half + CASTLE_WALL_LENGTH_HALF,
                -CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite: sprite.clone(),
            ..default()
        })
        .insert(Collider::Solid);

    // player 1 castle wall right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(
                ARENA_SIZE.width_half - CASTLE_WALL_LENGTH_HALF,
                -CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite: sprite.clone(),
            ..default()
        })
        .insert(Collider::Solid);

    // player 2 castle wall left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(
                -ARENA_SIZE.width_half + CASTLE_WALL_LENGTH_HALF,
                CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite: sprite.clone(),
            ..default()
        })
        .insert(Collider::Solid);

    // player 2 castle wall right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(
                ARENA_SIZE.width_half - CASTLE_WALL_LENGTH_HALF,
                CASTLE_WALL_Y_TRANSLATION,
                0.0
            ),
            sprite,
            ..default()
        })
        .insert(Collider::Solid);
}