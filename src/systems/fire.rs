use amethyst::{
    core::transform::Transform,
    core::Named,
 //   ecs::Resources,
    ecs::prelude::{Entity, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System },
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

use crate::resources::{
    AssetType,
    SpriteSheetList,
};

use crate::state::Projectile;
use crate::state::Planet;

pub struct FireZeMissilesSystem;

impl<'s> System<'s> for FireZeMissilesSystem {
    type SystemData = (
        Entities<'s>,
//        WriteStorage<'s, Projectile>,
        ReadStorage<'s, Planet>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, SpriteSheetList>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, /*projectiles,*/ planets, lazy_update, sprite_sheet_list, input) = data;
        let fire_input = input.action_is_down("shoot").expect("shoot action exists");

        let planet = planets.join().next().unwrap();

        if fire_input {
            if let Some((x, y)) = input.mouse_position() {
                let size = 25.;
                let mx = x - size;
                // @TODO Fix using screen dimensions
                let my = 600. - y;

                let displacement = (mx - planet.x + size / 2.,
                                    my - planet.y + size / 2.);
                let angle = displacement.1.atan2(displacement.0);
                // let distance = displacement.0.powi(2) + displacement.1.powi(2);

                let force = 0.000275;//(g * 2.0 / distance).min(0.00005);
                let acceleration = (angle.cos() * force * 2.0e4, angle.sin() * force * 2.0e4);
                let start_offset = (35.0 * angle.cos(), 35.0 * angle.sin());

            let projectile_sprite_sheet_handle = {
                sprite_sheet_list.get(AssetType::Projectile).unwrap().clone()
            };

            let x = planet.x + start_offset.0;
            let y = planet.y + start_offset.1;
            let projectile: Entity = entities.create();
            lazy_update.insert(projectile, Projectile {
                velocity: [ acceleration.0, acceleration.1 ],
                acc: [ 0., 0. ],
                radius: 1.,
                x: x,
                y: y,
            });
            let sprite_render = SpriteRender {
                sprite_sheet: projectile_sprite_sheet_handle,
                sprite_number: 0,
            };
            lazy_update.insert(projectile, Named::new("Projectile"));
            lazy_update.insert(projectile, sprite_render);
            let mut new_transform = Transform::default();
            new_transform.set_translation_xyz(x, y, 0.);
            lazy_update.insert(projectile, new_transform);
            }
        }
    }
}

/*
 * For use when the force is variable.
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    }
    else if value > max {
        max
    }
    else {
        value
    }
}
*/
