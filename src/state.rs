use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use log::info;

pub struct Yasg;

impl SimpleState for Yasg {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        world.register::<Projectile>();
        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        init_sprites(world, &sprites, &dimensions);
        load_sprite2(world, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

// Merge the sprite loading into one function.
fn load_sprite2(world: &mut World, dimensions: &ScreenDimensions) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/projectile.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/projectile.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    let sprite = SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0,
    };

    let x = dimensions.width() * 0.5;
    let y = dimensions.height() * 0.5;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    // Create an entity for each sprite and attach the `SpriteRender` as
    // well as the transform. If you want to add behaviour to your sprites,
    // you'll want to add a custom `Component` that will identify them, and a
    // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
    world
        .create_entity()
        .with(sprite.clone())
        .with(transform)
        .with(Projectile {
            velocity: [0.2, -0.02],
            radius: 1.,
            x: x,
            y: x,
        })
        .build();
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/planet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/planet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    vec!(SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0,
    })
}

fn init_sprites(world: &mut World, sprites: &[SpriteRender], _dimensions: &ScreenDimensions) {
    let mut x = 350.;
    let mut y = 250.;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    // Create an entity for each sprite and attach the `SpriteRender` as
    // well as the transform. If you want to add behaviour to your sprites,
    // you'll want to add a custom `Component` that will identify them, and a
    // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
    world
        .create_entity()
        .with(sprites[0].clone())
        .with(Planet {
            x: x,
            y: y,
        })
        .with(transform)
        .build();

    x = 550.;
    y = 150.;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    world
        .create_entity()
        .with(sprites[0].clone())
        .with(Planet {
            x: x,
            y: y,
        })
        .with(transform)
        .build();
}

#[derive(Debug)]
pub struct Planet {
    pub x: f32,
    pub y: f32,
}

impl Component for Planet {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
