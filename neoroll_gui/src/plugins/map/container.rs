use bevy::prelude::*;
use neoroll_world::map::part::MapPart;

use crate::camera::{camera_map_area, PlayerCamera};

use super::updater::MapUpdater;

#[derive(Event)]
pub struct MapPartContainerRefreshed;
#[derive(Event)]
pub struct MapPartContainerNeedRefresh;

#[derive(Resource)]
pub struct MapPartContainer(pub MapPart);

impl Default for MapPartContainer {
    fn default() -> Self {
        Self(MapPart::empty())
    }
}

impl MapPartContainer {
    pub fn map_part(&self) -> &MapPart {
        &self.0
    }
}

pub fn refresh_map_part_container(
    player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
    map_updater: ResMut<MapUpdater>,
    mut map_part_container: ResMut<MapPartContainer>,
    mut map_part_container_need_change: EventReader<MapPartContainerNeedRefresh>,
    mut map_part_container_change: EventWriter<MapPartContainerRefreshed>,
) {
    if !map_part_container_need_change.is_empty() {
        map_part_container_need_change.clear();

        let (_, camera, transform) = player_camera.single();
        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let scale = transform.scale;
        let area = camera_map_area(target, translation, scale);
        let area = area.resize(2, 2);

        map_updater.update(&mut map_part_container, area);
        map_part_container_change.send(MapPartContainerRefreshed);
    }
}
