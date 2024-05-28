use std::thread;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tileset::prelude::*;

use plugins::{inputs::UserInputsPlugin, map::MapDisplayPlugin, world::WorldDisplayPlugin};
use setup::setup_;
use neoroll_server::run::RunnerBuilder;

mod camera;
mod debug;
mod graphics;
mod layer;
mod plugins;
mod scene;
mod setup;

fn main() {
    // TODO: like in OpenCombat, permit remote server instead embedded server
    thread::spawn(|| {
        RunnerBuilder::new().actions(vec![]).build().run();
    });

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
