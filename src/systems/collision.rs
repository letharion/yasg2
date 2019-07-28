use amethyst::{
    ecs::prelude::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::state::Projectile;
use crate::state::Planet;

pub struct ProjectileCollisionSystem;

impl<'s> System<'s> for ProjectileCollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Projectile>,
        ReadStorage<'s, Planet>,
    );

    fn run(&mut self, (entities, projectiles, planets): Self::SystemData) {
        for (entity, projectile) in (&entities, &projectiles).join() {
            for planet in planets.join() {
                let distance = hyp(planet.x - projectile.x, planet.y - projectile.y);
                if distance < 27.5 {
                    println!("collision happened");
                    // @TODO Fix error handling.
                    let _ = entities.delete(entity);
                }
            }
        }
    }
}

fn hyp (height: f32, width: f32) -> f32 {
  (width.powi(2) + height.powi(2)).sqrt()
}
