use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tileset::prelude::*;
use neoroll_world::map::MAP_TILE_FACTOR;

use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    graphics::{AlphaByScale, REGION_TILE_HEIGHT, REGION_TILE_WIDTH},
    plugins::map::{background::Background, container::MapPartContainer, element::Element},
    scene::ScenePoint, utils::EventReaderShortcuts,
};

use super::{
    background::MapBackgroundNeedResize,
    container::MapPartContainerRefreshed,
    lake::Lake,
    tileset::{element_tile_name, spawn, MapResources, MAP_TILESET_NAME},
};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn refresh_map_display(
    camera: Query<
        (&SceneItemsCamera, &Camera, &mut Transform),
        (With<SceneItemsCamera>, Without<BackgroundCamera>),
    >,
    mut map_container_refreshed: EventReader<MapPartContainerRefreshed>,
    elements_query: Query<Entity, With<Element>>,
    lakes_query: Query<Entity, With<Lake>>,
    mut background_query: Query<
        (&mut Visibility, &mut Sprite),
        (With<Background>, Without<Camera>),
    >,
    tilesets: Tilesets,
    map_part_container: Res<MapPartContainer>,
    mut commands: Commands,
) {
    let (_, _, camera_transform) = camera.single();
    let scale = camera_transform.scale;
    let (mut background_visibility, mut background_sprite) = background_query.single_mut();

    if let Some(tileset) = tilesets.get_by_name(MAP_TILESET_NAME) {
        if map_container_refreshed.has_been_set()
        {
            let atlas = tileset.atlas();
            let map_part = map_part_container.map_part();

            elements_query
                .iter()
                .for_each(|e| commands.entity(e).despawn());
            lakes_query
                .iter()
                .for_each(|e| commands.entity(e).despawn());

            let alpha = AlphaByScale::map();
            let color = alpha.color(scale.x);

            if !alpha.display(scale.x) {
                *background_visibility = Visibility::Hidden;
                return;
            }

            *background_visibility = Visibility::Visible;
            background_sprite.color = color;

            // Elements
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

            // Lakes
            // TODO : code for test now, will be according to area
            for lake in map_part_container.0.lakes() {
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(ScenePoint::from_world_point(lake.first().unwrap()).into());
                for point in lake {
                    path_builder.line_to(ScenePoint::from_world_point(point).into());
                }
                path_builder.close();
                let path = path_builder.build();
                commands.spawn((
                    ShapeBundle { path, ..default() },
                    Stroke::new(Color::BLUE, 5.0),
                    Fill::color(Color::BLUE),
                    Lake,
                ));
            }
        }
    }
}

pub fn resize_background(
    mut resize: EventReader<MapBackgroundNeedResize>,
    map: Res<MapResources>,
    images: Res<Assets<Image>>,
    mut background: Query<&mut Transform, (With<Background>, Without<Camera>)>,
) {
    if let Some(event) = resize
        .iter()
        .collect::<Vec<&MapBackgroundNeedResize>>()
        .last()
    {
        if let Some(background_handle) = &map.background {
            if let Some(background_image) = images.get(background_handle) {
                let mut background_transform = background.single_mut();
                let target = &event.0;

                let background_scale = Vec3::new(
                    target.width / background_image.size().x,
                    target.height / background_image.size().y,
                    1.,
                );
                background_transform.scale = background_scale;
            }
        }
    }
}
