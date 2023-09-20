use bevy::prelude::*;
use neoroll_world::space::part::WorldPart;

use crate::camera::{camera_world_area, SceneItemsCamera};

use super::updater::WorldUpdater;

#[derive(Event)]
pub struct WorldPartContainerRefreshed;
#[derive(Event)]
pub struct WorldPartContainerNeedRefresh;

#[derive(Resource)]
pub struct WorldPartContainer(pub WorldPart);

impl Default for WorldPartContainer {
    fn default() -> Self {
        Self(WorldPart::empty())
    }
}

impl WorldPartContainer {
    pub fn world_part(&self) -> &WorldPart {
        &self.0
    }
}

pub fn refresh_world_part_container(
    player_camera: Query<(&SceneItemsCamera, &Camera, &mut Transform)>,
    world_updater: ResMut<WorldUpdater>,
    mut world_part_container: ResMut<WorldPartContainer>,
    mut world_part_container_need_change: EventReader<WorldPartContainerNeedRefresh>,
    mut world_part_container_change: EventWriter<WorldPartContainerRefreshed>,
) {
    if !world_part_container_need_change.is_empty() {
        world_part_container_need_change.clear();

        let (_, camera, transform) = player_camera.single();
        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let scale = transform.scale;
        let area = camera_world_area(target, translation, scale);
        let area = area.resize(30, 30);

        // FIXME BS NOW : fix a max area size to retrieve only needed area when on
        // map (to be able to compute path finding without download all world ...)
        world_updater.update(&mut world_part_container, area);
        world_part_container_change.send(WorldPartContainerRefreshed);
    }
}
