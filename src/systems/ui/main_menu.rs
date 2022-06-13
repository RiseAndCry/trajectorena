use crate::prelude::*;
use bevy::app::AppExit;

// todo use a library for UI
pub fn main_menu_system(
    mut state: ResMut<State<AppState>>,
    menu_data: Res<MenuData>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                match entity {
                    e if e == menu_data.play_button_entity => state.set(AppState::InGame).unwrap(),
                    e if e == menu_data.quit_button_entity => exit.send(AppExit),
                    _ => (),
                }
            }
            Interaction::Hovered => {
                *color = BUTTON_COLOR_HOVERED.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR_DEFAULT.into();
            }
        }
    }
}
