#![warn(clippy::pedantic)]

use crate::prelude::*;

pub fn spawn_score_text(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(SCREEN_WIDTH / 2.0 + ARENA_SIZE.0 / 2.0 + 10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Health: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText);
}

pub fn score_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    castle_health: ResMut<CastleHealth>
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = castle_health.health.to_string();
    }
}