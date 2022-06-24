use crate::prelude::*;

#[derive(Component)]
pub struct Spell {
    pub is_on_hold: bool,
    pub player: Player,
}

impl Spell {
    pub fn new(player: Player) -> Self {
        Spell {
            is_on_hold: false,
            player
        }
    }
}

#[derive(Bundle)]
pub struct SpellBundle {
    pub spell: Spell,
    pub movement: Movement,
    pub despawnable: Despawnable,
    #[bundle]
    pub sprite: SpriteBundle,
}