use crate::prelude::*;

pub fn state_update_system(
    mut app_state: ResMut<State<AppState>>,
    castle_health: ResMut<CastleHealth>
) {
    if castle_health.health <= 0 && *app_state.current() != AppState::GameOver {
        app_state.set(AppState::GameOver).unwrap();
    }
}