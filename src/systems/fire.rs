use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::Named,
    ecs::Resources,
    ecs::prelude::{Entity, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::resources::{
    AssetType,
    SpriteSheetList,
};

use crate::state::Projectile;
use crate::state::Planet;

use log::info;
pub struct FireZeMissilesSystem;

impl<'s> System<'s> for FireZeMissilesSystem {
    type SystemData = (
        Entities<'s>,
//        WriteStorage<'s, Projectile>,
  //      ReadStorage<'s, Planet>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, SpriteSheetList>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, /*projectiles, planet,*/ lazy_update, sprite_sheet_list, transform, time) = data;

        if (time.delta_seconds() * 100000.) as i32 % 100 == 0 {
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
 //       lazy_update.insert(projectile, two_dim_object);
   /*     lazy_update.insert(
            projectile,
            Collider::new(
                Vector2::new(shoot_start_position, marine_bottom + 48.),
                BoundingRect::new(ctx.x_correction, ctx.map_width, 352., 0.),
            ),
        );
        lazy_update.insert(projectile, Collidee::default());*/
        lazy_update.insert(projectile, sprite_render);
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);
        lazy_update.insert(projectile, transform);
 /*       lazy_update.insert(projectile, motion);
        lazy_update.insert(projectile, direction);
        lazy_update.insert(projectile, Transparent);*/
        }
    }
}
