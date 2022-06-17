use crate::prelude::*;

pub fn spawn_player_1_health_text(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let p1_position = Rect {
        bottom: Val::Px(0.0),
        left: Val::Px(SCREEN_SIZE.width_half + ARENA_SIZE.width_half + 10.0),
        ..default()
    };
    let p1_sections = vec![
        create_text_section(asset_server, "Health: ".to_string(), FONT_TEXT, Color::WHITE),
        create_text_section(asset_server, MAX_PLAYER_HEALTH.to_string(), FONT_HEALTH, Color::BLUE),
    ];
    commands
        .spawn_bundle(create_text_bundle(p1_position, p1_sections))
        .insert(HealthText {player: Player::One});
}

pub fn spawn_player_2_health_text(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let p2_position = Rect {
        top: Val::Px(0.0),
        left: Val::Px(SCREEN_SIZE.width_half + ARENA_SIZE.width_half + 10.0),
        ..default()
    };
    let p2_sections = vec![
        create_text_section(asset_server, "Health: ".to_string(), FONT_TEXT, Color::WHITE),
        create_text_section(asset_server, MAX_PLAYER_HEALTH.to_string(), FONT_HEALTH, Color::RED),
    ];
    commands
        .spawn_bundle(create_text_bundle(p2_position, p2_sections))
        .insert(HealthText {player: Player::Two});
}

fn create_text_section(
    asset_server: &Res<AssetServer>,
    value: String,
    font: &str,
    color: Color
) -> TextSection {
    TextSection {
        value,
        style: TextStyle {
            font: asset_server.load(font),
            font_size: 40.0,
            color,
        },
    }
}

fn create_text_bundle(position: Rect<Val>, sections: Vec<TextSection>) -> TextBundle {
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position,
                ..default()
            },
            text: Text {
                sections,
                ..default()
            },
            ..default()
        }
}
