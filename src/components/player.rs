use crate::prelude::*;

pub const MAX_PLAYER_HEALTH: u32 = 3;
pub const PLAYER_VELOCITY: f32 = 1200.0;
pub struct PlayerSize {
    pub width: f32,
    pub width_half: f32,
    pub height: f32,
    pub height_half: f32,
}
pub const PLAYER_SIZE: PlayerSize = PlayerSize {
    width: 24.0,
    width_half: 12.0,
    height: 24.0,
    height_half: 12.0,
};
impl From<PlayerSize> for Vec2 {
    fn from(_: PlayerSize) -> Self {
        Vec2::new(PLAYER_SIZE.width, PLAYER_SIZE.height)
    }
}

#[derive(Component, PartialEq)]
pub enum Player {
    One,
    Two,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub movement: Movement,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(player: Player, starting_position: Vec3, color: Color) -> Self {
        Self {
            player,
            health: Health::new(MAX_PLAYER_HEALTH),
            movement: Movement::new(Vec3::from((PLAYER_VELOCITY, PLAYER_VELOCITY, 0.0))),
            sprite: SpriteBundle {
                transform: Transform::from_translation(starting_position),
                sprite: Sprite {
                    custom_size: Some(Vec2::from(PLAYER_SIZE)),
                    color,
                    ..default()
                },
                ..default()
            },
        }
    }
}