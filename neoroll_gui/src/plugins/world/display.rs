use bevy::prelude::*;
use bevy_tileset::prelude::*;

use crate::{
    camera::SceneItemsCamera,
    graphics::AlphaByScale,
    plugins::world::{container::WorldPartContainer, region::RegionTile},
    scene::ScenePoint,
};

use super::{
    container::WorldPartContainerRefreshed,
    resolver::LayersResolver,
    tileset::{spawn, WORLD_TILESET_NAME},
};

pub fn refresh_world_display(
    camera: Query<(&SceneItemsCamera, &Camera, &mut Transform)>,
    mut world_container_refreshed: EventReader<WorldPartContainerRefreshed>,
    tiles: Query<Entity, With<RegionTile>>,
    tilesets: Tilesets,
    world_container: Res<WorldPartContainer>,
    commands: Commands,
) {
    let (_, _, camera_transform) = camera.single();

    if let Some(tileset) = tilesets.get_by_name(WORLD_TILESET_NAME) {
        if world_container_refreshed
            .iter()
            .collect::<Vec<&WorldPartContainerRefreshed>>()
            .last()
            .is_some()
        {
            re_spawn_world(
                tiles,
                world_container,
                tileset,
                commands,
                camera_transform.scale,
            );
        }
    }
}

pub fn re_spawn_world(
    tiles: Query<Entity, With<RegionTile>>,
    world_container: Res<WorldPartContainer>,
    tileset: &Tileset,
    mut commands: Commands,
    scale: Vec3,
) {
    let atlas = tileset.atlas();
    let world_part = world_container.world_part();

    tiles.iter().for_each(|e| commands.entity(e).despawn());

    let alpha = AlphaByScale::world();

    if !alpha.display(scale.x) {
        return;
    }

    let color = alpha.color(scale.x);
    for (((point, ground), (_, floor)), (_, structure)) in world_part
        .grounds()
        .iter()
        .zip(world_part.floors().iter())
        .zip(world_part.structures())
    {
        let tiles = LayersResolver.resolve(ground, floor, structure);
        for tile in tiles {
            if let Some((tile_index, _)) = &tileset.select_tile(&tile.0) {
                let scene_point = ScenePoint::from_world_point(point);
                commands.spawn(spawn(atlas, tile_index, scene_point.into(), color));
            }
        }
    }

    let (human_tile_index, _) = &tileset.select_tile("Human").unwrap();
    commands.spawn(spawn(atlas, human_tile_index, (0., 0., 0.).into(), color));
}
