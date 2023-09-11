use bevy::prelude::*;
use bevy_tileset::prelude::*;

use graphics::tileset::RegionTileset;
use input::{inputs, InputState};
use setup::setup_;
use world::{display_world, WorldContainer};

pub mod camera;
pub mod graphics;
pub mod input;
pub mod setup;
pub mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TilesetPlugin::default()))
        .init_resource::<InputState>()
        .init_resource::<RegionTileset>()
        .init_resource::<WorldContainer>()
        .add_systems(Startup, setup_)
        .add_systems(Update, (display_world, inputs))
        .run();
}
