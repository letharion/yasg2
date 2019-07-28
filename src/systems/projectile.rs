use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::state::Projectile;
use crate::state::Planet;

pub struct MoveProjectilesSystem;

impl<'s> System<'s> for MoveProjectilesSystem {
    type SystemData = (
        WriteStorage<'s, Projectile>,
        ReadStorage<'s, Planet>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut projectiles, planets, mut locals, time): Self::SystemData) {
        for (projectile, local) in (&mut projectiles, &mut locals).join() {
            for planet in planets.join() {
                let (x, y) = gravity(planet, projectile);
                projectile.velocity[0] += x * time.delta_seconds();
                projectile.velocity[1] += y * time.delta_seconds();
                projectile.x += projectile.velocity[0];
                projectile.y += projectile.velocity[1];
                local.set_translation_x(projectile.x);
                local.set_translation_y(projectile.y);
            }
        }
    }
}

// https://codereview.stackexchange.com/questions/88566/gravity-model-for-a-simulator/88568#88568
fn gravity(planet: &Planet, projectile: &Projectile) -> (f32, f32) {
    let g = 1000.;
    let dx = planet.x - projectile.x;
    let dy = planet.y - projectile.y;
    let r  = hyp(dx, dy);
    let inv_r3 = 1.0 / (r * r * r);

    (dx * inv_r3 * g, dy * inv_r3 * g)
}

fn hyp (height: f32, width: f32) -> f32 {
  (width.powi(2) + height.powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(hyp(1., 1.), 2_f32.sqrt());
    }
}
