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
    creature::{spawn_creature, CreatureComponent, CreaturesMap},
    resolver::LayersResolver,
    tileset::{spawn_tile, WORLD_TILESET_NAME},
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
                commands.spawn(spawn_tile(atlas, tile_index, scene_point.into(), color));
            }
        }
    }

    for creature in world_part.creatures().values() {
        let (human_tile_index, _) = &tileset.select_tile("Human").unwrap();
        let scene_point = ScenePoint::from_world_point(creature.point());

        let entity = commands
            .spawn(spawn_creature(
                *creature.id(),
                atlas,
                human_tile_index,
                scene_point.into(),
                color,
            ))
            .id();
        creatures_map.insert(*creature.id(), entity);
    }
}
