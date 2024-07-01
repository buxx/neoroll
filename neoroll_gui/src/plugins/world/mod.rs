pub mod illustration;
use bevy::prelude::*;
use container::WorldPartContainerRefreshed;
use creature::{CreaturesMap, ProgressMap};
use display::refresh_progress_display;

use self::{
    container::{refresh_world_part_container, WorldPartContainer, WorldPartContainerNeedRefresh},
    display::refresh_world_display,
    init::init_world,
    tileset::WorldTileset,
};

pub mod container;
pub mod creature;
pub mod display;
pub mod init;
pub mod region;
pub mod resolver;
pub mod tileset;

pub struct WorldDisplayPlugin;

impl Plugin for WorldDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldTileset>()
            .init_resource::<WorldPartContainer>()
            .init_resource::<CreaturesMap>()
            .init_resource::<ProgressMap>()
            .add_event::<WorldPartContainerNeedRefresh>()
            .add_event::<WorldPartContainerRefreshed>()
            .add_systems(
                Update,
                (
                    init_world,
                    refresh_world_part_container,
                    refresh_world_display,
                    refresh_progress_display,
                ),
            );
    }
}
