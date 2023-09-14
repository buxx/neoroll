use bevy::prelude::*;

use self::container::WorldPartContainerNeedRefresh;

pub mod container;
pub mod region;
pub mod updater;

// TODO : Better way to run once ? Do it in setup_ ? But how to access player_camera ?
pub fn init_world(
    mut world_part_container_need_change: EventWriter<WorldPartContainerNeedRefresh>,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    world_part_container_need_change.send(WorldPartContainerNeedRefresh);
    *has_ran = true;
}
