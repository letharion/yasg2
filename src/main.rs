use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod state;
mod systems;
mod resources;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(app_root.join("resources/bindings_config.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)?
        .with(systems::MoveProjectilesSystem, "projectile_system", &[])
        .with(systems::ProjectileCollisionSystem, "collision_system", &[])
        .with(systems::FireZeMissilesSystem, "firezemissiles_system", &[])
    ;

    let mut game = Application::new(resources, state::Yasg, game_data)?;
    game.run();

    Ok(())
}
