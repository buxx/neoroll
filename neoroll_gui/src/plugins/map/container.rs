use bevy::prelude::*;
use neoroll_world::map::part::MapPart;

use crate::camera::{camera_map_area, BackgroundCamera, SceneItemsCamera};

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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn refresh_map_part_container(
    camera: Query<(
        &SceneItemsCamera,
        &Camera,
        &mut Transform,
        (With<SceneItemsCamera>, Without<BackgroundCamera>),
    )>,
    map_updater: ResMut<MapUpdater>,
    mut map_container: ResMut<MapPartContainer>,
    mut map_container_need_refresh: EventReader<MapPartContainerNeedRefresh>,
    mut map_container_refreshed: EventWriter<MapPartContainerRefreshed>,
) {
    if !map_container_need_refresh
        .iter()
        .collect::<Vec<&MapPartContainerNeedRefresh>>()
        .is_empty()
    {
        let (_, camera, transform, _) = camera.single();
        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let scale = transform.scale;
        let area = camera_map_area(target, translation, scale);
        let area = area.resize(2, 2);

        map_updater.update(&mut map_container, area);
        map_container_refreshed.send(MapPartContainerRefreshed);
    }
}
