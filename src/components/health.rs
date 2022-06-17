use crate::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

impl Health {
    pub fn new(value: u32) -> Self {
        Health {
            value,
        }
    }
}