use bevy_tileset::prelude::*;
use neoroll_world::{
    space::{ColI, RowI},
    state::World,
};

use bevy::prelude::*;

use crate::graphics::tileset::{spawn, REGION_TILESET_NAME};

#[derive(Resource, Default)]
pub struct WorldContainer(pub World);

#[derive(Component)]
pub struct RegionTile;

pub fn init_world(
    world: Res<WorldContainer>,
    tilesets: Tilesets,
    commands: Commands,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    if let Some(tileset) = tilesets.get_by_name(REGION_TILESET_NAME) {
        display_world(world, tileset, commands);
        *has_ran = true;
    }
}

pub fn display_world(world: Res<WorldContainer>, tileset: &Tileset, mut commands: Commands) {
    let atlas = tileset.atlas();
    for row in 0..world.0.lines() {
        for col in 0..world.0.columns() {
            if let Some((tile_index, _)) =
                &tileset.select_tile(&world.0.region(RowI(row), ColI(col)).tile().to_string())
            {
                commands.spawn(spawn(
                    atlas,
                    tile_index,
                    Vec3::new(col as f32 * 16.0, row as f32 * 16.0, 0.0),
                ));
            }
        }
    }
}

pub fn remove_world(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<Entity, With<RegionTile>>,
    mut commands: Commands,
) {
    if keyboard_input.just_released(KeyCode::H) {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}
