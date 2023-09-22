use bevy::{prelude::*, window::WindowResized};

use crate::plugins::map::background::MapBackgroundNeedResize;
use crate::plugins::map::container::MapPartContainerNeedRefresh;
use crate::plugins::world::container::WorldPartContainerNeedRefresh;

pub fn on_window_resize(
    mut window_resize: EventReader<WindowResized>,
    mut world_refresh: EventWriter<WorldPartContainerNeedRefresh>,
    mut map_refresh: EventWriter<MapPartContainerNeedRefresh>,
    mut map_background_refresh: EventWriter<MapBackgroundNeedResize>,
) {
    if let Some(event) = window_resize
        .into_iter()
        .collect::<Vec<&WindowResized>>()
        .last()
    {
        world_refresh.send(WorldPartContainerNeedRefresh);
        map_refresh.send(MapPartContainerNeedRefresh);
        map_background_refresh.send(MapBackgroundNeedResize((*event).clone()));
    }
}
