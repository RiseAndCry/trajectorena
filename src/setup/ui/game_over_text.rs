use crate::prelude::*;

pub fn spawn_game_over_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                // todo this is a bit hacky, clean up
                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                position: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Percent(10.0),
                    bottom: Val::Auto
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::with_section(
                    "Game Over !",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::RED,
                    },
                    TextAlignment::default(),
                ),
                ..default()
            });
        })
        .insert(GameOverText);
}
