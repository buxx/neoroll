use bevy::{prelude::*, window::WindowResized};

use crate::plugins::map::container::MapPartContainerNeedRefresh;
use crate::plugins::world::container::WorldPartContainerNeedRefresh;

// FIXME BS NOW : only on concerned scale
pub fn refresh_world_on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut refresh_writer: EventWriter<WorldPartContainerNeedRefresh>,
) {
    if !resize_reader.is_empty() {
        resize_reader.clear();
        refresh_writer.send(WorldPartContainerNeedRefresh);
    }
}

// FIXME BS NOW : only on concerned scale
pub fn refresh_map_on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut refresh_writer: EventWriter<MapPartContainerNeedRefresh>,
) {
    if !resize_reader.is_empty() {
        resize_reader.clear();
        refresh_writer.send(MapPartContainerNeedRefresh);
    }
}
