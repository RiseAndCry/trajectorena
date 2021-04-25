#![warn(clippy::pedantic)]

mod resources;
mod components;
mod systems;

mod prelude {
    pub use bevy::{
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
    };

    pub use crate::resources::*;
    pub use crate::components::*;
    pub use crate::systems::*;

    // todo handle resizing, different resolutions
    pub const SCREEN_WIDTH: f32 = 1280.0;
    pub const SCREEN_HEIGHT: f32 = 720.0;
}

use prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Trajectorena".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .insert_resource(SpellCooldown::new())
        .insert_resource(CastleHealth::new())
        .init_resource::<ButtonMaterials>()

        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(main_menu_setup.system()))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(main_menu_system.system()))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu.system()))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(in_game_setup.system()))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(health_update_system.system())
                .with_system(spell_movement_system.system())
                .with_system(spell_collision_system.system())
                .with_system(player_movement_system.system())
                .with_system(player_shooting_system.system())
                .with_system(despawn_system.system())
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(despawn_everything_system.system())
        )
        .add_system_set(
            // todo show `Main menu` button
            SystemSet::on_enter(AppState::GameOver).with_system(game_over_setup.system())
        )

        .add_system(state_update_system.system())

        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    let button_entity = spawn_play_button(&mut commands, &asset_server, &button_materials);

    commands.insert_resource(MenuData { button_entity });

    commands.spawn_bundle(UiCameraBundle::default());
}

fn in_game_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    // todo move spawning outside of systems (maybe 'setup' folder) ?
    spawn_health_text(&mut commands, &asset_server);
    spawn_arena_bounds(&mut commands, &mut materials);
    spawn_castles(&mut commands, &mut materials);
    spawn_castle_walls(&mut commands, &mut materials);
    spawn_player(&mut commands, &mut materials);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn game_over_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    spawn_game_over_text(&mut commands, &mut materials, &asset_server);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}