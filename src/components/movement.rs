#![warn(clippy::pedantic)]

use crate::prelude::*;

#[derive(Component)]
pub struct Movement {
    pub velocity: Vec3,
}

// todo add default trait
impl Movement {
    pub fn new(velocity: Vec3) -> Self {
        Movement {
            velocity
        }
    }
}