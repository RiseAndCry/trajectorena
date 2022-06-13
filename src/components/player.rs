use crate::prelude::*;

const PLAYER_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            speed: PLAYER_SPEED,
        }
    }
}