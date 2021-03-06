use crate::prelude::*;

pub fn spell_collision_system(
    mut spell_query: Query<(&Spell, &Transform, &mut Movement, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (_, spell_transform, mut spell_movement, sprite) in spell_query.iter_mut() {
        let spell_size = sprite.custom_size.expect("Sprite size must be set");

        for (_, collider, transform, sprite) in collider_query.iter() {
            // collision might not be detected if an object is moving too fast. Solutions:
            // 1. reduce the object movement speed
            // 2. increase the size of colliding object(s)
            // 3. lower MOVEMENT_TIME_STEP - the smaller this timeStep is, the less distance
            // the object will move, thus making it easier for collision to detect it.
            let collision = collide(
                spell_transform.translation,
                spell_size,
                transform.translation,
                sprite.custom_size.expect("Sprite size must be set"),
            );
            if let Some(collision) = collision {
                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = spell_movement.velocity.x > 0.0,
                    Collision::Right => reflect_x = spell_movement.velocity.x < 0.0,
                    Collision::Top => reflect_y = spell_movement.velocity.y < 0.0,
                    Collision::Bottom => reflect_y = spell_movement.velocity.y > 0.0,
                    Collision::Inside => ()
                }

                if reflect_x {
                    spell_movement.velocity.x = -spell_movement.velocity.x;
                }
                if reflect_y {
                    spell_movement.velocity.y = -spell_movement.velocity.y;
                }

                if *collider == Collider::Solid {
                    break;
                }
            }
        }
    }
}