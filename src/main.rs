#![warn(clippy::all)]

mod setup;
mod resources;
mod components;
mod systems;

mod prelude {
    pub use bevy::{
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
    };

    pub use crate::setup::*;
    pub use crate::resources::*;
    pub use crate::components::*;
    pub use crate::systems::*;

    // todo handle resizing, different resolutions
    pub const SCREEN_WIDTH: f32 = 1280.0;
    pub const SCREEN_HEIGHT: f32 = 720.0;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Trajectorena".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .add_state(AppState::Menu)
        // <><--- MainMenu ---><>
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(main_menu_setup))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(main_menu_system))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_system))
        // <><--- InGame ---><>
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(in_game_setup))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(health_update_system)
                .with_system(spell_movement_system)
                .with_system(spell_collision_system)
                .with_system(spell_despawn_system)
                .with_system(player_movement_system)
                .with_system(player_shooting_system)
                .with_system(state_update_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(despawn_system)
        )
        // <><--- GameOver ---><>
        .add_system_set(
            SystemSet::on_enter(AppState::GameOver)
                .with_system(game_over_setup)
                .with_system(main_menu_setup)
        )
        .add_system_set(
            SystemSet::on_update(AppState::GameOver)
                .with_system(main_menu_system)
        )
        .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(despawn_system))

        // todo ESC should open main menu
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let play_button_entity = spawn_play_button(&mut commands, &asset_server);
    let quit_button_entity = spawn_quit_button(&mut commands, &asset_server);

    commands.insert_resource(MenuData { play_button_entity, quit_button_entity });

    commands.spawn_bundle(UiCameraBundle::default());
}

fn in_game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(CastleHealth::new());
    commands.insert_resource(SpellCooldown::new());

    spawn_health_text(&mut commands, &asset_server);
    spawn_arena_bounds(&mut commands);
    spawn_castles(&mut commands);
    spawn_castle_walls(&mut commands);
    spawn_player(&mut commands);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn game_over_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    spawn_game_over_text(&mut commands, &asset_server);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}