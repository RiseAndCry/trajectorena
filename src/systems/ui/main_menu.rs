#![warn(clippy::pedantic)]

use crate::prelude::*;
use bevy::app::AppExit;

pub fn main_menu_system(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonMaterials>,
    menu_data: Res<MenuData>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                // todo why does match not work here ?
                if entity.id() == menu_data.play_button_entity.id() {
                    state.set(AppState::InGame).unwrap();
                }
                if entity.id() == menu_data.quit_button_entity.id() {
                    exit.send(AppExit);
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
