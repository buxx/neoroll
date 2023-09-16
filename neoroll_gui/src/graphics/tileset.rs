use bevy::prelude::*;
use bevy_tileset::prelude::*;
use neoroll_world::entity::{floor::Floor, ground::Ground, structure::Structure};

use crate::plugins::world::region::RegionTile;

use super::TileName;

pub const REGION_TILESET_NAME: &str = "Regions";

pub fn ground_tile_name(ground: &Ground) -> TileName {
    match ground {
        Ground::Soil => TileName("Soil".to_string()),
    }
}
pub fn floor_tile_name(ground: &Floor) -> TileName {
    match ground {
        Floor::Nothing => TileName("Nothing".to_string()),
        Floor::ShortGrass => TileName("ShortGrass".to_string()),
    }
}

pub fn structure_tile_name(structure: &Structure) -> TileName {
    match structure {
        Structure::Nothing => TileName("Nothing".to_string()),
        Structure::BigLeafTree => TileName("BigLeafTree".to_string()),
    }
}

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
