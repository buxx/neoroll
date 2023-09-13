use bevy::prelude::*;
use bevy_tileset::prelude::*;

use camera::debug_camera;
use graphics::tileset::RegionTileset;
use input::{inputs, manual_refresh_world_part_container, InputState};
use setup::setup_;
use window::on_window_resize;
use world::{
    init_world, refresh_world_display, refresh_world_part_container, WorldPartContainer,
    WorldPartContainerNeedRefresh, WorldPartContainerRefreshed, WorldReader,
};

pub mod camera;
pub mod graphics;
pub mod input;
pub mod scene;
pub mod setup;
pub mod window;
pub mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TilesetPlugin::default()))
        .init_resource::<InputState>()
        .init_resource::<RegionTileset>()
        .init_resource::<WorldReader>()
        .init_resource::<WorldPartContainer>()
        .add_event::<WorldPartContainerNeedRefresh>()
        .add_event::<WorldPartContainerRefreshed>()
        .add_systems(Startup, setup_)
        .add_systems(
            Update,
            (
                inputs,
                init_world,
                refresh_world_part_container,
                refresh_world_display,
                manual_refresh_world_part_container,
                on_window_resize,
                debug_camera,
            ),
        )
        .run();
}
