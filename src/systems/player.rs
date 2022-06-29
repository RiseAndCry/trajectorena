use crate::prelude::*;
use bevy::input::mouse::MouseButtonInput;

const CURSOR_VICINITY_FOR_SPELL_DETECTION: (f32, f32) = (100.0, 100.0);

// maybe store controls as resources ?
struct PlayerControls {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}
const P1_CONTROLS: PlayerControls = PlayerControls {
    up: KeyCode::W,
    down: KeyCode::S,
    left: KeyCode::A,
    right: KeyCode::D,
};
const P2_CONTROLS: PlayerControls = PlayerControls {
    up: KeyCode::Up,
    down: KeyCode::Down,
    left: KeyCode::Left,
    right: KeyCode::Right,
};

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &Movement, &mut Transform)>,
) {
    for (player, movement, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let mut controls = P1_CONTROLS;
        if *player == Player::Two {
            controls = P2_CONTROLS
        }

        handle_player_movement(&keyboard_input, movement, translation, &controls);
        restrict_player_movement(player, translation);
    }
}

fn handle_player_movement(
    keyboard_input: &Res<Input<KeyCode>>,
    movement: &Movement,
    translation: &mut Vec3,
    controls: &PlayerControls
) {
    for key in keyboard_input.get_pressed() {
        match key {
            k if k == &controls.left => translation.x -= movement.velocity.x * MOVEMENT_TIME_STEP,
            k if k == &controls.right => translation.x += movement.velocity.x * MOVEMENT_TIME_STEP,
            k if k == &controls.up => translation.y += movement.velocity.y * MOVEMENT_TIME_STEP,
            k if k == &controls.down => translation.y -= movement.velocity.y * MOVEMENT_TIME_STEP,
            _ => (),
        }
    }
}

fn restrict_player_movement(player: &Player, translation: &mut Vec3) {
    // bound the player within the horizontal arena bounds
    translation.x = translation.x
        .min(ARENA_SIZE.width_half - PLAYER_SIZE.width_half)
        .max(-ARENA_SIZE.width_half + PLAYER_SIZE.width_half);

    // bound the player within the vertical arena bounds
    match player {
        Player::One => {
            // bottom castle
            translation.y = translation.y
                .min(-CASTLE_WALL_Y_TRANSLATION - ARENA_WALL_THICKNESS_HALF - PLAYER_SIZE.width_half)
                .max(-ARENA_SIZE.height_half + CASTLE_WALL_THICKNESS_HALF + PLAYER_SIZE.width_half);
        },
        Player::Two => {
            // top castle
            translation.y = translation.y
                .min(ARENA_SIZE.height_half - CASTLE_WALL_THICKNESS_HALF - PLAYER_SIZE.width_half)
                .max(CASTLE_WALL_Y_TRANSLATION + ARENA_WALL_THICKNESS_HALF + PLAYER_SIZE.width_half);
        },
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
            spawn_spell(commands, player_transform, direction, Player::One);
            spell_cooldown.timer.reset();
        }
    }
}

// RMB press holds the selected spell and release will shoot it in a new direction
// todo update readme
pub fn spell_holding_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut evr_cursor: EventReader<CursorMoved>,
    mut spells_query: Query<(&mut Spell, &Sprite, &Transform)>,
) {
    // check if collision happens between spell and cursor vicinity
    if mouse_button_input.pressed(MouseButton::Right) {
        // todo use a radius
        // todo detect closest instead of the first occurrence
        for cursor in evr_cursor.iter() {
            let cursor_world_pos = cursor.position - Vec2::new(SCREEN_SIZE.width_half, SCREEN_SIZE.height_half);
            for (mut spell, sprite, transform) in spells_query.iter_mut() {
                if collide(
                    Vec3::from((cursor_world_pos, 0.0)),
                    Vec2::from(CURSOR_VICINITY_FOR_SPELL_DETECTION),
                    transform.translation,
                    sprite.custom_size.expect("Sprite size must be set"),
                ).is_some() {
                    spell.is_on_hold = true;
                };
            }
        }
    }

    // release the spell
    if mouse_button_input.just_released(MouseButton::Right) {
        // todo change trajectory
        for (mut spell, _, _) in spells_query.iter_mut() {
            if spell.is_on_hold {
                spell.is_on_hold = false;
            }
        }
    }
}