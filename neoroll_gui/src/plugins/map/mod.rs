use bevy::prelude::*;
use container::MapPartContainerRefreshed;

use self::{
    background::MapBackgroundNeedResize,
    container::{refresh_map_part_container, MapPartContainer, MapPartContainerNeedRefresh},
    display::{refresh_map_display, resize_background},
    init::init_map,
    tileset::MapResources,
};

pub mod background;
pub mod container;
pub mod display;
pub mod element;
pub mod init;
pub mod lake;
pub mod tileset;

pub struct MapDisplayPlugin;

impl Plugin for MapDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapResources>()
            .init_resource::<MapPartContainer>()
            .add_event::<MapPartContainerNeedRefresh>()
            .add_event::<MapPartContainerRefreshed>()
            .add_event::<MapBackgroundNeedResize>()
            .add_systems(
                Update,
                (
                    init_map,
                    refresh_map_part_container,
                    refresh_map_display,
                    resize_background,
                ),
            );
    }
}
