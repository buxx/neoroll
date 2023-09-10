use bevy::prelude::*;
use bevy_tileset::prelude::*;

use camera::move_camera;
use graphics::tileset::{show_tileset, RegionTileset};
use setup::setup_;

pub mod camera;
pub mod graphics;
pub mod setup;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TilesetPlugin::default()))
        .init_resource::<RegionTileset>()
        .add_systems(Startup, setup_)
        .add_systems(Update, (show_tileset, move_camera))
        .run();
}
