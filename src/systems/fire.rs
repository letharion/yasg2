use amethyst::{
    core::transform::Transform,
    core::Named,
 //   ecs::Resources,
    ecs::prelude::{Entity, Entities, LazyUpdate, Read, ReadExpect, System },
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

use crate::resources::{
    AssetType,
    SpriteSheetList,
};

use crate::state::Projectile;
//use crate::state::Planet;

pub struct FireZeMissilesSystem;

impl<'s> System<'s> for FireZeMissilesSystem {
    type SystemData = (
        Entities<'s>,
//        WriteStorage<'s, Projectile>,
  //      ReadStorage<'s, Planet>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, SpriteSheetList>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, /*projectiles, planet,*/ lazy_update, sprite_sheet_list, input) = data;
        let fire_input = input.action_is_down("shoot").expect("shoot action exists");

        if fire_input {
            let x = 350.;
            let y = 500.;
            let projectile_sprite_sheet_handle = {
                sprite_sheet_list.get(AssetType::Projectile).unwrap().clone()
            };

            let projectile: Entity = entities.create();
            lazy_update.insert(projectile, Projectile {
                velocity: [ 0.6, -1.15 ],
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
