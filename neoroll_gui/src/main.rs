use bevy::prelude::*;
use bevy_tileset::prelude::*;

use camera::move_camera;
use graphics::tileset::{display_world, RegionTileset};
use setup::setup_;

pub mod camera;
pub mod graphics;
pub mod setup;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TilesetPlugin::default()))
        .init_resource::<RegionTileset>()
        .add_systems(Startup, setup_)
        .add_systems(Update, (display_world, move_camera))
        .run();
}
