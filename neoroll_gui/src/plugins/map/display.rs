use bevy::prelude::*;
use bevy_tileset::prelude::*;
use neoroll_world::map::MAP_TILE_FACTOR;

use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    graphics::{AlphaByScale, REGION_TILE_HEIGHT, REGION_TILE_WIDTH},
    plugins::map::{
        background::Background,
        container::{MapPartContainer, MapPartContainerRefreshed},
        element::Element,
    },
    scene::ScenePoint,
};

use super::tileset::{element_tile_name, spawn, MapResources, MAP_TILESET_NAME};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn refresh_map_display(
    player_camera: Query<
        (&SceneItemsCamera, &Camera, &mut Transform),
        (With<SceneItemsCamera>, Without<BackgroundCamera>),
    >,
    mut map_part_container_change: EventReader<MapPartContainerRefreshed>,
    element_tiles_query: Query<Entity, With<Element>>,
    mut background_query: Query<
        (&mut Visibility, &mut Transform),
        (With<Background>, Without<Camera>),
    >,
    tilesets: Tilesets,
    map_part_container: ResMut<MapPartContainer>,
    map_resources: Res<MapResources>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
) {
    let (_, camera, camera_transform) = player_camera.single();
    let scale = camera_transform.scale;
    let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
    let (mut background_visibility, mut background_transform) = background_query.single_mut();

    if let Some(tileset) = tilesets.get_by_name(MAP_TILESET_NAME) {
        if !map_part_container_change.is_empty() {
            map_part_container_change.clear();

            //////////////

            let atlas = tileset.atlas();
            let map_part = map_part_container.map_part();

            element_tiles_query
                .iter()
                .for_each(|e| commands.entity(e).despawn());

            let alpha = AlphaByScale::map();

            if !alpha.display(scale.x) {
                *background_visibility = Visibility::Hidden;
                return;
            }

            if *background_visibility == Visibility::Hidden {
                *background_visibility = Visibility::Visible;

                if let Some(background_handle) = &map_resources.background {
                    if let Some(background_image) = images.get(background_handle) {
                        let background_scale = Vec3::new(
                            target.x as f32 / background_image.size().x,
                            target.y as f32 / background_image.size().y,
                            1.,
                        );
                        background_transform.scale = background_scale;
                    }
                }
            }

            // Elements
            let color = alpha.color(scale.x);
            for (point, sector) in map_part.sectors() {
                if let Some(sector) = sector {
                    for (relative_point, element) in sector.elements() {
                        if let Some((tile_index, _)) =
                            &tileset.select_tile(&element_tile_name(element).0)
                        {
                            let sector_scene_point = ScenePoint::from_world_point(&point.into());
                            let scene_point = sector_scene_point.apply(
                                (REGION_TILE_WIDTH * MAP_TILE_FACTOR) as f32 * relative_point.0,
                                (REGION_TILE_HEIGHT * MAP_TILE_FACTOR) as f32 * relative_point.1,
                            );
                            commands.spawn(spawn(atlas, tile_index, scene_point.into(), color));
                        }
                    }
                }
            }
        }
    }
}
