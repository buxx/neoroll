use bevy::prelude::*;

use crate::plugins::{
    map::container::MapPartContainerNeedRefresh, world::container::WorldPartContainerNeedRefresh,
};

#[derive(Event)]
pub struct DraggedScreen(pub Vec3);

pub fn on_dragged_screen(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut dragged_screen: EventReader<DraggedScreen>,
    mut world_part_container_need_change: EventWriter<WorldPartContainerNeedRefresh>,
    mut map_part_container_need_change: EventWriter<MapPartContainerNeedRefresh>,
) {
    let mut camera = camera.single_mut();
    let mut need_refresh = false;

    for event in dragged_screen.iter() {
        camera.translation.x -= event.0.x;
        camera.translation.y += event.0.y;
        need_refresh = true;
    }

    if need_refresh {
        // Avoid ugly pixels by translate only on entire pixels
        camera.translation = camera.translation.round();
        // Trigger world refresh
        world_part_container_need_change.send(WorldPartContainerNeedRefresh);
        map_part_container_need_change.send(MapPartContainerNeedRefresh);
    }
}
