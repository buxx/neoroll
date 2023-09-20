use bevy::{prelude::*, render::view::RenderLayers};
use bevy_tileset::prelude::*;
use neoroll_world::entity::{floor::Floor, ground::Ground, structure::Structure};

use crate::{graphics::TileName, layer::LAYER_SCENE_ITEMS, plugins::world::region::RegionTile};

pub const WORLD_TILESET_NAME: &str = "World";

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
pub struct WorldTileset {
    pub handle: Option<Handle<Tileset>>,
}

pub fn spawn(
    atlas: &Handle<TextureAtlas>,
    tile_index: &TileIndex,
    point: Vec3,
    color: Color,
) -> (RegionTile, SpriteSheetBundle, RenderLayers) {
    (
        RegionTile,
        match tile_index {
            TileIndex::Standard(index) => {
                let mut sprite = TextureAtlasSprite::new(*index);
                sprite.color = color;
                SpriteSheetBundle {
                    transform: Transform {
                        translation: point,
                        ..Default::default()
                    },
                    sprite,
                    texture_atlas: atlas.clone(),
                    ..Default::default()
                }
            }
            TileIndex::Animated(_start, _end, _speed) => {
                todo!()
            }
        },
        RenderLayers::layer(LAYER_SCENE_ITEMS),
    )
}
