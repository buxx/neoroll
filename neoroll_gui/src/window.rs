use bevy::{prelude::*, window::WindowResized};

use crate::world::WorldPartContainerNeedRefresh;

pub fn on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut refresh_writer: EventWriter<WorldPartContainerNeedRefresh>,
) {
    if !resize_reader.is_empty() {
        resize_reader.clear();
        refresh_writer.send(WorldPartContainerNeedRefresh);
    }
}
