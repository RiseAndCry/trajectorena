use crate::prelude::*;
use bevy::input::mouse::MouseButtonInput;

pub struct PlayerSize {
    pub width: f32,
    pub width_half: f32,
    pub height: f32,
    pub height_half: f32,
}
pub const PLAYER_SIZE: PlayerSize = PlayerSize {
    width: 24.0,
    width_half: 12.0,
    height: 24.0,
    height_half: 12.0,
};
impl From<PlayerSize> for Vec2 {
    fn from(_: PlayerSize) -> Self {
        Vec2::new(PLAYER_SIZE.width, PLAYER_SIZE.height)
    }
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &Movement, &mut Transform)>,
) {
    for (_, movement, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        for key in keyboard_input.get_pressed() {
            match key {
                KeyCode::A => translation.x -= movement.velocity.x * MOVEMENT_TIME_STEP,
                KeyCode::D => translation.x += movement.velocity.x * MOVEMENT_TIME_STEP,
                KeyCode::W => translation.y += movement.velocity.y * MOVEMENT_TIME_STEP,
                KeyCode::S => translation.y -= movement.velocity.y * MOVEMENT_TIME_STEP,
                _ => (),
            }
        }

        // bound the player within the walls
        translation.x = translation.x
            .min(ARENA_SIZE.width_half - PLAYER_SIZE.width_half)
            .max(-ARENA_SIZE.width_half + PLAYER_SIZE.width_half);
        translation.y = translation.y
            .min(-CASTLE_WALL_Y_TRANSLATION - ARENA_WALL_THICKNESS_HALF - PLAYER_SIZE.width_half)
            .max(-ARENA_SIZE.height_half + CASTLE_WALL_THICKNESS_HALF + PLAYER_SIZE.width_half);
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
    for (player, transform) in player_query.iter() {
        match player {
            Player::One => handle_player_1_shooting(
                &time,
                &mut commands,
                &mut evr_mouse_btn,
                &mut evr_cursor,
                &mut spell_cooldown,
                transform
            ),
            Player::Two => (), // todo
        }
    }
}

fn handle_player_1_shooting(
    time: &Res<Time>,
    commands: &mut Commands,
    evr_mouse_btn: &mut EventReader<MouseButtonInput>,
    evr_cursor: &mut EventReader<CursorMoved>,
    spell_cooldown: &mut ResMut<SpellCooldown>,
    player_transform: &Transform
) {
    spell_cooldown.timer.tick(time.delta());
    if !spell_cooldown.timer.finished() {
        return;
    }

    let mut direction = Vec3::ZERO;
    for ev in evr_cursor.iter() {
        // cursor to world coordinates, since bevy does not yet have built-in function for this
        // (https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
        let cursor_world_pos = ev.position - Vec2::new(SCREEN_SIZE.width, SCREEN_SIZE.height) / 2.0;
        let cursor_world_pos: Vec3 = Vec3::new(cursor_world_pos.x, cursor_world_pos.y, 0.0);
        direction = cursor_world_pos - player_transform.translation;
    }

    for ev in evr_mouse_btn.iter() {
        if !ev.state.is_pressed() && ev.button == MouseButton::Left {
            spawn_spell(commands, player_transform, direction);
            spell_cooldown.timer.reset();
        }
    }
}