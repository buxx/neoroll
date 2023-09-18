use bevy::prelude::*;
use bevy_tileset::prelude::*;
use neoroll_world::map::MAP_TILE_FACTOR;

use crate::{
    camera::PlayerCamera,
    plugins::map::{
        container::{MapPartContainer, MapPartContainerRefreshed},
        element::Element,
    },
    scene::ScenePoint,
};

use super::{
    tileset::map::{element_tile_name, spawn, MAP_TILESET_NAME},
    AlphaByScale, REGION_TILE_HEIGHT, REGION_TILE_WIDTH,
};

pub fn refresh_map_display(
    player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
    mut map_part_container_change: EventReader<MapPartContainerRefreshed>,
    region_tiles_query: Query<Entity, With<Element>>,
    tilesets: Tilesets,
    map_part_container: ResMut<MapPartContainer>,
    commands: Commands,
) {
    let (_, _, camera_transform) = player_camera.single();

    if let Some(tileset) = tilesets.get_by_name(MAP_TILESET_NAME) {
        if !map_part_container_change.is_empty() {
            map_part_container_change.clear();
            re_spawn_map(
                region_tiles_query,
                map_part_container,
                tileset,
                commands,
                camera_transform.scale,
            );
        }
    }
}

pub fn re_spawn_map(
    element_tiles_query: Query<Entity, With<Element>>,
    map_part_container: ResMut<MapPartContainer>,
    tileset: &Tileset,
    mut commands: Commands,
    scale: Vec3,
) {
    let atlas = tileset.atlas();
    let map_part = map_part_container.map_part();

    element_tiles_query
        .iter()
        .for_each(|e| commands.entity(e).despawn());

    let alpha = AlphaByScale::map();

    if !alpha.display(scale.x) {
        return;
    }
    let color = Color::Rgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: alpha.alpha(scale.x),
    };

    for (point, sector) in map_part.sectors() {
        if let Some(sector) = sector {
            for (relative_point, element) in sector.elements() {
                if let Some((tile_index, _)) = &tileset.select_tile(&element_tile_name(element).0) {
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
