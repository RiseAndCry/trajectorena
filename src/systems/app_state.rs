#![warn(clippy::pedantic)]

use crate::prelude::*;

pub fn spawn_game_over_text(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Default::default(),
                text: Text::with_section(
                    "Game Over !",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::RED,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(GameOverText);
}

pub fn state_update_system(
    mut app_state: ResMut<State<AppState>>,
    castle_health: ResMut<CastleHealth>
) {
    if castle_health.health <= 0 && *app_state.current() != AppState::GameOver {
        app_state.set(AppState::GameOver).unwrap();
    }
}