use bevy::prelude::*;
use bevy_tileset::prelude::*;

use plugins::{inputs::UserInputsPlugin, world::WorldDisplayPlugin};
use setup::setup_;

mod camera;
mod graphics;
mod plugins;
mod scene;
mod setup;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TilesetPlugin::default(),
            UserInputsPlugin,
            WorldDisplayPlugin,
        ))
        .add_systems(Startup, setup_)
        .run();
}
