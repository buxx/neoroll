use bevy::prelude::*;

use super::container::WorldPartContainerNeedRefresh;

// TODO : Better way to run once ? Do it in setup_ ? But how to access player_camera ?
pub fn init_world(
    mut world_container_need_refresh: EventWriter<WorldPartContainerNeedRefresh>,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    world_container_need_refresh.send(WorldPartContainerNeedRefresh);
    *has_ran = true;
}
