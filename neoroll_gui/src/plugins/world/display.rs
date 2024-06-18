use bevy::prelude::*;
use bevy_tileset::prelude::*;

use crate::{
    camera::SceneItemsCamera,
    graphics::AlphaByScale,
    plugins::world::{container::WorldPartContainer, region::TileComponent},
    scene::ScenePoint,
};

use super::{
    container::WorldPartContainerRefreshed,
    creature::{
        spawn_creature, CreatureComponent, CreaturesMap, ProgressDone, ProgressMap, ProgressTotal,
    },
    resolver::LayersResolver,
    tileset::{material_tile_name, spawn_tile, WORLD_TILESET_NAME},
};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn refresh_world_display(
    world_part: Res<WorldPartContainer>,
    creatures_map: ResMut<CreaturesMap>,
    camera: Query<(&SceneItemsCamera, &Camera, &mut Transform)>,
    mut world_container_refreshed: EventReader<WorldPartContainerRefreshed>,
    tiles: Query<Entity, With<TileComponent>>,
    creatures: Query<Entity, With<CreatureComponent>>,
    tilesets: Tilesets,
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
                world_part,
                creatures_map,
                tiles,
                creatures,
                tileset,
                commands,
                camera_transform.scale,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn re_spawn_world(
    world_part: Res<WorldPartContainer>,
    mut creatures_map: ResMut<CreaturesMap>,
    tiles: Query<Entity, With<TileComponent>>,
    creatures: Query<Entity, With<CreatureComponent>>,
    tileset: &Tileset,
    mut commands: Commands,
    scale: Vec3,
) {
    let atlas = tileset.atlas();
    let world_part = world_part.world_part();

    // Creatures mapping will be completely refilled
    creatures_map.clear();
    tiles.iter().for_each(|e| commands.entity(e).despawn());
    creatures.iter().for_each(|e| commands.entity(e).despawn());

    let alpha = AlphaByScale::world();

    if !alpha.display(scale.x) {
        return;
    }

    let color = alpha.color(scale.x);
    for ((((point, ground), (_, floor)), (_, materials)), (_, structure)) in world_part
        .grounds()
        .iter()
        .zip(world_part.floors().iter())
        .zip(world_part.materials().iter())
        .zip(world_part.structures())
    {
        let tiles = LayersResolver.resolve(ground, floor, structure);
        let scene_point = ScenePoint::from_world_point(point);

        for tile in tiles {
            if let Some((tile_index, _)) = &tileset.select_tile(&tile.0) {
                commands.spawn(spawn_tile(atlas, tile_index, scene_point.into(), color));
            }
        }

        // TODO: display side-at-side ?
        if let Some(materials) = materials {
            for (material, _) in materials {
                if let Some((tile_index, _)) = &tileset.select_tile(&material_tile_name(material).0) {
                    commands.spawn(spawn_tile(atlas, tile_index, scene_point.into(), color));
                }
            }
        }
    }

    for creature in world_part.creatures().values() {
        let (human_tile_index, _) = &tileset.select_tile("Human").unwrap();
        let scene_point = ScenePoint::from_world_point(creature.point());

        creatures_map.insert(
            *creature.id(),
            commands
                .spawn(spawn_creature(
                    *creature.id(),
                    atlas,
                    human_tile_index,
                    scene_point.into(),
                    color,
                ))
                .id(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn refresh_progress_display(
    camera: Query<(&SceneItemsCamera, &Camera, &mut Transform)>,
    mut commands: Commands,
    progress_total: Query<Entity, With<ProgressTotal>>,
    progress_done: Query<Entity, With<ProgressDone>>,
    mut progress_map: ResMut<ProgressMap>,
) {
    let (_, _, camera_transform) = camera.single();
    let alpha = AlphaByScale::world();

    if !alpha.display(camera_transform.scale.x) {
        progress_total
            .iter()
            .for_each(|e| commands.entity(e).despawn());
        progress_done
            .iter()
            .for_each(|e| commands.entity(e).despawn());
        progress_map.clear();
    }
}
