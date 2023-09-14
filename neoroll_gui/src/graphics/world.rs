use bevy::prelude::*;
use bevy_tileset::prelude::*;

use crate::{
    plugins::world::{
        container::{WorldPartContainer, WorldPartContainerRefreshed},
        region::RegionTile,
    },
    scene::ScenePoint,
};

use super::tileset::{spawn, REGION_TILESET_NAME};

pub fn refresh_world_display(
    mut world_part_container_change: EventReader<WorldPartContainerRefreshed>,
    region_tiles_query: Query<Entity, With<RegionTile>>,
    tilesets: Tilesets,
    world_part_container: ResMut<WorldPartContainer>,
    commands: Commands,
) {
    if let Some(tileset) = tilesets.get_by_name(REGION_TILESET_NAME) {
        if !world_part_container_change.is_empty() {
            world_part_container_change.clear();
            re_spawn_world(region_tiles_query, world_part_container, tileset, commands);
        }
    }
}

pub fn re_spawn_world(
    region_tiles_query: Query<Entity, With<RegionTile>>,
    world_part_container: ResMut<WorldPartContainer>,
    tileset: &Tileset,
    mut commands: Commands,
) {
    let atlas = tileset.atlas();
    let world_part = world_part_container.world_part();

    region_tiles_query
        .iter()
        .for_each(|e| commands.entity(e).despawn());

    for (world_point, region) in world_part.regions() {
        if let Some(region) = region {
            if let Some((tile_index, _)) = &tileset.select_tile(&region.tile().to_string()) {
                let scene_point = ScenePoint::from_world_point(world_point);
                commands.spawn(spawn(atlas, tile_index, scene_point.into()));
            }
        }
    }
}
