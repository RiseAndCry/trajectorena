use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new(speed: f32) -> Self {
        Player {
            speed,
        }
    }
}