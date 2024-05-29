use bevy::prelude::*;
use container::WorldPartContainerRefreshed;

use self::{
    container::{refresh_world_part_container, WorldPartContainer, WorldPartContainerNeedRefresh},
    display::refresh_world_display,
    init::init_world,
    tileset::WorldTileset,
    updater::WorldUpdater,
};

pub mod container;
pub mod display;
pub mod init;
pub mod region;
pub mod resolver;
pub mod tileset;
pub mod updater;

pub struct WorldDisplayPlugin;

impl Plugin for WorldDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldTileset>()
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
                ),
            );
    }
}
