#![warn(clippy::pedantic)]

use crate::prelude::*;
use bevy::input::mouse::MouseButtonInput;

const PLAYER_SIZE: (f32, f32) = (16.0, 16.0);
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

pub fn player_movement_system(
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

pub fn player_shooting_system(
    time: Res<Time>,
    mut commands: Commands,
    mut evr_mouse_btn: EventReader<MouseButtonInput>,
    mut evr_cursor: EventReader<CursorMoved>,
    mut spell_cooldown: ResMut<SpellCooldown>,
    player_query: Query<(&Player, &Transform)>,
) {
    spell_cooldown.timer.tick(time.delta());

    let mut direction = Vec3::ZERO;
    let (_, player_transform) = player_query.single()
        .expect("Second player is not implemented yet");

    if !spell_cooldown.timer.finished() {
        return;
    }
    for ev in evr_cursor.iter() {
        // cursor to world coordinates, since bevy does not yet have built-in function for this
        // (https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
        let cursor_world_pos = ev.position - Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT) / 2.0;
        let cursor_world_pos: Vec3 = Vec3::new(cursor_world_pos.x, cursor_world_pos.y, 0.0);
        direction = cursor_world_pos - player_transform.translation;
    }

    for ev in evr_mouse_btn.iter() {
        if !ev.state.is_pressed() && ev.button == MouseButton::Left {
            spawn_spell(&mut commands, player_transform, direction);
            spell_cooldown.timer.reset();
        }
    }
}