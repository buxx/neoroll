use bevy::prelude::*;
use bevy_tileset::prelude::*;

use crate::{
    camera::PlayerCamera,
    graphics::AlphaByScale,
    plugins::world::{
        container::{WorldPartContainer, WorldPartContainerRefreshed},
        region::RegionTile,
    },
    scene::ScenePoint,
};

use super::{
    resolver::LayersResolver,
    tileset::world::{spawn, WORLD_TILESET_NAME},
};

pub fn refresh_world_display(
    player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
    mut world_part_container_change: EventReader<WorldPartContainerRefreshed>,
    region_tiles_query: Query<Entity, With<RegionTile>>,
    tilesets: Tilesets,
    world_part_container: ResMut<WorldPartContainer>,
    commands: Commands,
) {
    let (_, _, camera_transform) = player_camera.single();

    if let Some(tileset) = tilesets.get_by_name(WORLD_TILESET_NAME) {
        if !world_part_container_change.is_empty() {
            world_part_container_change.clear();
            re_spawn_world(
                region_tiles_query,
                world_part_container,
                tileset,
                commands,
                camera_transform.scale,
            );
        }
    }
}

pub fn re_spawn_world(
    region_tiles_query: Query<Entity, With<RegionTile>>,
    world_part_container: ResMut<WorldPartContainer>,
    tileset: &Tileset,
    mut commands: Commands,
    scale: Vec3,
) {
    let atlas = tileset.atlas();
    let world_part = world_part_container.world_part();

    region_tiles_query
        .iter()
        .for_each(|e| commands.entity(e).despawn());

    let alpha = AlphaByScale::world();

    if !alpha.display(scale.x) {
        return;
    }
    let color = Color::Rgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: alpha.alpha(scale.x),
    };

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
}
