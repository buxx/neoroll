use bevy::prelude::*;

use super::container::MapPartContainerNeedRefresh;

// TODO : Better way to run once ? Do it in setup_ ? But how to access player_camera ?
pub fn init_map(
    mut world_part_container_need_change: EventWriter<MapPartContainerNeedRefresh>,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    world_part_container_need_change.send(MapPartContainerNeedRefresh);
    *has_ran = true;
}
