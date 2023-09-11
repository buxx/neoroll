use bevy_tileset::prelude::Tilesets;
use neoroll_world::{
    space::{ColI, RowI},
    state::World,
};

use bevy::prelude::*;

use crate::graphics::tileset::{spawn, REGION_TILESET_NAME};

#[derive(Resource, Default)]
pub struct WorldContainer(pub World);

pub fn display_world(
    world: Res<WorldContainer>,
    tilesets: Tilesets,
    mut commands: Commands,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    if let Some(tileset) = tilesets.get_by_name(REGION_TILESET_NAME) {
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

        *has_ran = true;
    }
}
