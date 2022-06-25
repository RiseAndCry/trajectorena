#![warn(clippy::all)]

mod setup;
mod resources;
mod components;
mod systems;
mod events;

mod prelude {
    pub use bevy::{
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
        core::FixedTimestep,
    };

    pub use crate::setup::*;
    pub use crate::resources::*;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::events::*;

    // todo handle resizing, different resolutions
    pub struct ScreenSize {
        pub width: f32,
        pub width_half: f32,
        pub height: f32,
        pub height_half: f32,
    }
    pub const SCREEN_SIZE: ScreenSize = ScreenSize {
        width: 1280.0,
        width_half: 640.0,
        height: 720.0,
        height_half: 360.0,
    };

    pub const MOVEMENT_TIME_STEP: f32 = 1.0 / 240.0;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Trajectorena".to_string(),
            width: SCREEN_SIZE.width,
            height: SCREEN_SIZE.height,
            ..default()
        })
        .add_state(AppState::Menu)
        .add_event::<SpellOutOfBounds>()
        // <><--- MainMenu ---><>
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(main_menu_setup))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(main_menu_system))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_system))
        // <><--- InGame ---><>
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(in_game_setup))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_movement_system) // this might need to be moved to fixedTimeStep
                .with_system(player_shooting_system)
                .with_system(spell_holding_system)
                .with_system(spell_despawn_system)
                .with_system(state_update_system)
                .with_system(reduce_player_health_when_spell_goes_out_of_bounds)
        )
        // movement and collision systems need to be handled on fixed time step so that collision
        // is calculated correctly - https://github.com/bevyengine/bevy/issues/1240
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(FixedTimestep::step(MOVEMENT_TIME_STEP as f64))
                .with_system(spell_movement_system)
                .with_system(spell_collision_system)
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
    commands.insert_resource(SpellCooldown::new());

    spawn_arena_bounds(&mut commands);
    spawn_castles(&mut commands);
    spawn_castle_walls(&mut commands);
    spawn_player_1(&mut commands);
    spawn_player_2(&mut commands);
    spawn_player_1_health_text(&mut commands, &asset_server);
    spawn_player_2_health_text(&mut commands, &asset_server);

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