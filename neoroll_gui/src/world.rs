use bevy_tileset::prelude::*;
use neoroll_world::state::{EntireWorld, WorldArea, WorldPart};

use bevy::prelude::*;

use crate::{
    camera::{camera_world_area, PlayerCamera},
    graphics::tileset::{spawn, REGION_TILESET_NAME},
    scene::ScenePoint,
};

#[derive(Resource, Default)]
pub struct WorldPartContainer(pub WorldPart);

impl WorldPartContainer {
    pub fn world_part(&self) -> &WorldPart {
        &self.0
    }
}

#[derive(Resource, Default)]
pub struct WorldReader {
    // TODO : For now, store entire world here to simply develop
    pub world: Option<EntireWorld>,
}

impl WorldReader {
    pub fn update(&self, world_part: &mut WorldPartContainer, area: WorldArea) {
        // TODO : here will be network stuff
        if let Some(world) = &self.world {
            // TODO : Instead recreate entire part, update current by grabbing new tiles, and removing not required enough
            world_part.0 = WorldPart::from_world(world, area);
        }
    }
}

#[derive(Component)]
pub struct RegionTile;

#[derive(Event)]
pub struct WorldPartContainerRefreshed;
#[derive(Event)]
pub struct WorldPartContainerNeedRefresh;

// TODO : Better way to run once ? Do it in setup_ ? But how to access player_camera ?
pub fn init_world(
    mut world_part_container_need_change: EventWriter<WorldPartContainerNeedRefresh>,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    world_part_container_need_change.send(WorldPartContainerNeedRefresh);
    *has_ran = true;
}

pub fn refresh_world_part_container(
    player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
    world_reader: ResMut<WorldReader>,
    mut world_part_container: ResMut<WorldPartContainer>,
    mut world_part_container_need_change: EventReader<WorldPartContainerNeedRefresh>,
    mut world_part_container_change: EventWriter<WorldPartContainerRefreshed>,
) {
    if !world_part_container_need_change.is_empty() {
        world_part_container_need_change.clear();

        let (_, camera, transform) = player_camera.single();
        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let area = camera_world_area(target, translation);

        world_reader.update(&mut world_part_container, area);
        world_part_container_change.send(WorldPartContainerRefreshed);
    }
}

pub fn refresh_world_display(
    // player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
    // world_reader: ResMut<WorldReader>,
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
