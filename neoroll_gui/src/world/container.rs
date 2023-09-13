use bevy::prelude::*;
use neoroll_world::state::WorldPart;

use crate::camera::{camera_world_area, PlayerCamera};

use super::reader::WorldReader;

#[derive(Event)]
pub struct WorldPartContainerRefreshed;
#[derive(Event)]
pub struct WorldPartContainerNeedRefresh;

#[derive(Resource, Default)]
pub struct WorldPartContainer(pub WorldPart);

impl WorldPartContainer {
    pub fn world_part(&self) -> &WorldPart {
        &self.0
    }
}

pub fn refresh_world_part_container(
    player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
    world_reader: ResMut<WorldReader>,
    mut world_part_container: ResMut<WorldPartContainer>,
    mut world_part_container_need_change: EventReader<WorldPartContainerNeedRefresh>,
    mut world_part_container_change: EventWriter<WorldPartContainerRefreshed>,
) {
    if !world_part_container_need_change.is_empty() {
        world_part_container_need_change.clear();

        let (_, camera, transform) = player_camera.single();
        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let area = camera_world_area(target, translation);

        world_reader.update(&mut world_part_container, area);
        world_part_container_change.send(WorldPartContainerRefreshed);
    }
}
