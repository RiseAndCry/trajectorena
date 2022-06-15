use crate::prelude::*;

#[derive(Component)]
pub enum Player {
    One,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub movement: Movement,
    #[bundle]
    pub sprite: SpriteBundle,
}