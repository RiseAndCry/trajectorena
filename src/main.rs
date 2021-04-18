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
        .insert_resource(WindowDescriptor {
            title: "Trajectorena".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .insert_resource(SpellCooldown::new())
        .insert_resource(CastleHealth::new())
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_system(score_update_system.system())
        .add_system(spell_movement_system.system())
        .add_system(spell_collision_system.system())
        .add_system(player_movement_system.system())
        .add_system(player_shooting_system.system())
        .add_system(despawn_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    spawn_score_text(&mut commands, &asset_server);
    spawn_arena_bounds(&mut commands, &mut materials);
    spawn_castles(&mut commands, &mut materials);
    spawn_castle_walls(&mut commands, &mut materials);
    spawn_player(&mut commands, &mut materials);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}