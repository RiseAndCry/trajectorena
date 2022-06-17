use crate::prelude::*;

pub fn state_update_system(
    mut app_state: ResMut<State<AppState>>,
    query: Query<&Health>,
) {
    for health in query.iter() {
        // todo announce which player won
        if health.value == 0 && *app_state.current() != AppState::GameOver {
            app_state.set(AppState::GameOver).unwrap();
        }
    }
}