use bevy::prelude::*;

use super::container::MapPartContainerNeedRefresh;

// TODO : Better way to run once ? Do it in setup_ ? But how to access player_camera ?
pub fn init_map(
    mut has_ran: Local<bool>,
    mut world_part_container_need_change: EventWriter<MapPartContainerNeedRefresh>,
) {
    if *has_ran {
        return;
    }

    world_part_container_need_change.send(MapPartContainerNeedRefresh);
    *has_ran = true;
}
