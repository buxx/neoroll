use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tileset::prelude::*;

use plugins::{inputs::UserInputsPlugin, map::MapDisplayPlugin, world::WorldDisplayPlugin};
use setup::setup_;

mod camera;
mod debug;
mod graphics;
mod layer;
mod plugins;
mod scene;
mod setup;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ShapePlugin,
            TilesetPlugin::default(),
            UserInputsPlugin,
            WorldDisplayPlugin,
            MapDisplayPlugin,
        ))
        .add_systems(Startup, setup_)
        .run();
}
