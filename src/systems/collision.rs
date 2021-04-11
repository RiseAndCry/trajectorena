#![warn(clippy::pedantic)]

use crate::prelude::*;

pub fn spell_collision_system(
    mut spell_query: Query<(&Spell, &Transform, &mut Movement, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (_, spell_transform, mut spell_movement, sprite) in spell_query.iter_mut() {
        let spell_size = sprite.size;

        for (_, collider, transform, sprite) in collider_query.iter() {
            // collision is not detected if the object is moving too fast, so either it has to be
            // slower, or one of the colliding objects needs to be bigger in size.
            // https://github.com/bevyengine/bevy/issues/1240
            let collision = collide(
                spell_transform.translation,
                spell_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = spell_movement.velocity.x > 0.0,
                    Collision::Right => reflect_x = spell_movement.velocity.x < 0.0,
                    Collision::Top => reflect_y = spell_movement.velocity.y < 0.0,
                    Collision::Bottom => reflect_y = spell_movement.velocity.y > 0.0,
                }

                if reflect_x {
                    spell_movement.velocity.x = -spell_movement.velocity.x;
                }

                if reflect_y {
                    spell_movement.velocity.y = -spell_movement.velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is also in collision
                if *collider == Collider::Solid {
                    break;
                }
            }
        }
    }
}