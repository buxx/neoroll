use bevy::prelude::*;

use crate::camera::{BackgroundCamera, SceneItemsCamera};

use super::{
    background::Background, container::MapPartContainerNeedRefresh, tileset::MapResources,
};

// TODO : Better way to run once ? Do it in setup_ ? But how to access player_camera ?
pub fn init_map(
    mut has_ran: Local<bool>,
    scene_items_camera: Query<&Camera, (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
    map_resources: Res<MapResources>,
    images: Res<Assets<Image>>,
    mut background_query: Query<&mut Transform, (With<Background>, Without<Camera>)>,
    mut world_part_container_need_change: EventWriter<MapPartContainerNeedRefresh>,
) {
    if *has_ran {
        return;
    }

    if let Some(background_handle) = &map_resources.background {
        if let Some(background_image) = images.get(background_handle) {
            let camera = scene_items_camera.single();
            let mut background_transform = background_query.single_mut();
            let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));

            let background_scale = Vec3::new(
                target.x as f32 / background_image.size().x,
                target.y as f32 / background_image.size().y,
                1.,
            );
            background_transform.scale = background_scale;

            //
            world_part_container_need_change.send(MapPartContainerNeedRefresh);
            *has_ran = true;
        }
    }
}
