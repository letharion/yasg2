use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::Named,
    ecs::prelude::{Entity, Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
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
        WriteStorage<'s, Projectile>,
        ReadStorage<'s, Planet>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, SpriteSheetList>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, projectile, planet, lazy_update, sprite_sheet_list, transform, time) = data;

        info!("handling key event: {:?}", 1);
/*        let x = 100.;
        let y = 100.;
        let projectile_sprite_sheet_handle = {
            sprite_sheet_list.get(AssetType::Projectile).unwrap().clone()
        };
        let projectile: Entity = entities.create();
        lazy_update.insert(projectile, Projectile {
            velocity: [0.2, -0.02],
            radius: 1.,
            x: x,
            y: y,
        });
        let sprite_render = SpriteRender {
            sprite_sheet: projectile_sprite_sheet_handle,
            sprite_number: 0,
        };
        lazy_update.insert(projectile, Named::new("Projectile"));*/
 //       lazy_update.insert(projectile, two_dim_object);
   /*     lazy_update.insert(
            projectile,
            Collider::new(
                Vector2::new(shoot_start_position, marine_bottom + 48.),
                BoundingRect::new(ctx.x_correction, ctx.map_width, 352., 0.),
            ),
        );
        lazy_update.insert(projectile, Collidee::default());*/
//        lazy_update.insert(projectile, sprite_render);
 /*       lazy_update.insert(projectile, motion);
        lazy_update.insert(projectile, transform);
        lazy_update.insert(projectile, direction);
        lazy_update.insert(projectile, Transparent);*/
    }
}
