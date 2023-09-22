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

pub const OVER_SCREEN_TILES: isize = 30;

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
    camera: Query<(&SceneItemsCamera, &Camera, &mut Transform)>,
    world_updater: ResMut<WorldUpdater>,
    mut world_container: ResMut<WorldPartContainer>,
    mut world_container_need_refresh: EventReader<WorldPartContainerNeedRefresh>,
    mut world_container_refreshed: EventWriter<WorldPartContainerRefreshed>,
) {
    if !world_container_need_refresh
        .iter()
        .collect::<Vec<&WorldPartContainerNeedRefresh>>()
        .is_empty()
    {
        let (_, camera, transform) = camera.single();
        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let scale = transform.scale;
        let area = camera_world_area(target, translation, scale)
            .resize(OVER_SCREEN_TILES, OVER_SCREEN_TILES);

        // FIXME BS NOW : fix a max area size to retrieve only needed area when on
        // map (to be able to compute path finding without download all world ...)
        // For now, when very little zoom, a lot of tile are loaded !!
        world_updater.update(&mut world_container, area);
        world_container_refreshed.send(WorldPartContainerRefreshed);
    }
}
