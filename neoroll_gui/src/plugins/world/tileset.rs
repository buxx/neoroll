use bevy::{prelude::*, render::view::RenderLayers};
use bevy_tileset::prelude::*;
use neoroll_world::entity::{floor::Floor, ground::Ground, structure::Structure};
use neoroll_world::gameplay::material::Material as Material_;
use neoroll_world::gameplay::material::Resource;

use crate::{graphics::TileName, layer::LAYER_SCENE_ITEMS, plugins::world::region::TileComponent};

pub const WORLD_TILESET_NAME: &str = "World";

pub fn ground_tile_name(ground: &Ground) -> TileName {
    match ground {
        Ground::Soil => TileName("Soil".to_string()),
        Ground::FreshWater => TileName("FreshWater".to_string()),
    }
}

pub fn floor_tile_name(ground: &Floor) -> TileName {
    match ground {
        Floor::Nothing => TileName("Nothing".to_string()),
        Floor::ShortGrass => TileName("ShortGrass".to_string()),
        Floor::FruitBush(filled) => match filled.0 {
            0..=50 => TileName("Bush".to_string()),
            51..=128 => TileName("FruitBush1".to_string()),
            129..=255 => TileName("FruitBush2".to_string()),
        },
    }
}

pub fn material_tile_name(material: &Material_) -> TileName {
    match material {
        Material_::Resource(Resource::Food) => TileName("Apple".to_string()),
    }
}

pub fn structure_tile_name(structure: &Structure) -> TileName {
    match structure {
        Structure::Nothing => TileName("Nothing".to_string()),
        Structure::BigLeafTree => TileName("BigLeafTree".to_string()),
        Structure::FruitTree(filled) => match filled.0 {
            0..=50 => TileName("BigLeafTree".to_string()),
            51..=128 => TileName("FruitTree1".to_string()),
            129..=255 => TileName("FruitTree2".to_string()),
        },
        Structure::Campfire => TileName("Campfire".to_string()),
        Structure::Storage => TileName("Storage".to_string()),
    }
}

#[derive(Resource, Default)]
pub struct WorldTileset {
    pub handle: Option<Handle<Tileset>>,
}

pub fn spawn_tile(
    atlas: &Handle<TextureAtlas>,
    tile_index: &TileIndex,
    point: Vec3,
    color: Color,
) -> (TileComponent, SpriteSheetBundle, RenderLayers) {
    (
        TileComponent,
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
