use crate::prelude::*;

// todo centralize all colors
pub const BUTTON_COLOR_DEFAULT: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BUTTON_COLOR_HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);

pub struct ButtonSize {
    pub width: f32,
    pub width_half: f32,
    pub height: f32,
    pub height_half: f32,
}

pub const BUTTON_SIZE: ButtonSize = ButtonSize {
    width: 150.0,
    width_half: 75.0,
    height: 65.0,
    height_half: 65.0 / 2.0,
};

pub fn spawn_play_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let mut button_bundle = get_button_bundle();
    button_bundle.style.position.top = Val::Percent(35.0);

    spawn_button(commands, asset_server, button_bundle, "Play")
}

pub fn spawn_quit_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let mut button_bundle = get_button_bundle();
    button_bundle.style.position.top = Val::Percent(55.0);

    spawn_button(commands, asset_server, button_bundle, "Quit")
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    button_bundle: ButtonBundle,
    name: &str,
) -> Entity {
    let button_entity = commands
        .spawn_bundle(button_bundle)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    name,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    TextAlignment::default(),
                ),
                ..default()
            });
        })
        .id();

    button_entity
}

fn get_button_bundle() -> ButtonBundle {
    ButtonBundle {
        // todo try to use flex instead of this manual calculation (NodeBundle as in GameOver text)
        style: Style {
            size: Size::new(Val::Px(BUTTON_SIZE.width), Val::Px(BUTTON_SIZE.height)),
            // center button
            position: Rect {
                left: Val::Px(SCREEN_SIZE.width_half - BUTTON_SIZE.width_half),
                right: Val::Auto,
                top: Val::Percent(50.0),
                bottom: Val::Auto
            },
            position_type: PositionType::Absolute,
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        color: BUTTON_COLOR_DEFAULT.into(),
        ..default()
    }
}