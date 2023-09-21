use bevy::{prelude::*, window::WindowResized};

use crate::camera::{BackgroundCamera, SceneItemsCamera};
use crate::plugins::map::background::Background;
use crate::plugins::map::container::MapPartContainerNeedRefresh;
use crate::plugins::map::tileset::MapResources;
use crate::plugins::world::container::WorldPartContainerNeedRefresh;

// FIXME BS NOW : only on concerned scale
pub fn refresh_world_on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut refresh_writer: EventWriter<WorldPartContainerNeedRefresh>,
) {
    if !resize_reader.is_empty() {
        resize_reader.clear();
        refresh_writer.send(WorldPartContainerNeedRefresh);
    }
}

// FIXME BS NOW : only on concerned scale
pub fn refresh_map_on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    scene_items_camera: Query<&Camera, (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
    map_resources: Res<MapResources>,
    images: Res<Assets<Image>>,
    mut background_query: Query<&mut Transform, (With<Background>, Without<Camera>)>,
    mut refresh_writer: EventWriter<MapPartContainerNeedRefresh>,
) {
    if !resize_reader.is_empty() {
        resize_reader.clear();

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
            }
        }

        refresh_writer.send(MapPartContainerNeedRefresh);
    }
}
