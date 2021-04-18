#![warn(clippy::pedantic)]

pub const MAX_HEALTH_CASTLE: i32 = 3;

pub struct CastleHealth {
    pub health: i32,
}

impl CastleHealth {
    pub fn new() -> Self {
        CastleHealth {
            health: MAX_HEALTH_CASTLE,
        }
    }
}