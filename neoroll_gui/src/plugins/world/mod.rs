use bevy::prelude::*;

use crate::graphics::{tileset::RegionTileset, world::refresh_world_display};

use self::{
    container::{
        refresh_world_part_container, WorldPartContainer, WorldPartContainerNeedRefresh,
        WorldPartContainerRefreshed,
    },
    init::init_world,
    updater::WorldUpdater,
};

use super::inputs::window::refresh_world_on_window_resize;

pub mod container;
pub mod init;
pub mod region;
pub mod updater;

pub struct WorldDisplayPlugin;

impl Plugin for WorldDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RegionTileset>()
            .init_resource::<WorldUpdater>()
            .init_resource::<WorldPartContainer>()
            .add_event::<WorldPartContainerNeedRefresh>()
            .add_event::<WorldPartContainerRefreshed>()
            .add_systems(
                Update,
                (
                    init_world,
                    refresh_world_part_container,
                    refresh_world_display,
                    refresh_world_on_window_resize,
                ),
            );
    }
}
