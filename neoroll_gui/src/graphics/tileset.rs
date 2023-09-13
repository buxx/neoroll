use bevy::prelude::*;
use bevy_tileset::prelude::*;

use crate::world::region::RegionTile;

pub const REGION_TILESET_NAME: &str = "Regions";

#[derive(Resource, Default)]
pub struct RegionTileset {
    pub handle: Option<Handle<Tileset>>,
}

pub fn spawn(
    atlas: &Handle<TextureAtlas>,
    tile_index: &TileIndex,
    point: Vec3,
) -> (RegionTile, SpriteSheetBundle) {
    (
        RegionTile,
        match tile_index {
            TileIndex::Standard(index) => SpriteSheetBundle {
                transform: Transform {
                    translation: point,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(*index),
                texture_atlas: atlas.clone(),
                ..Default::default()
            },
            TileIndex::Animated(_start, _end, _speed) => {
                todo!()
            }
        },
    )
}
