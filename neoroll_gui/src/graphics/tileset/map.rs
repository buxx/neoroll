use bevy::prelude::*;

use bevy_tileset::prelude::{TileIndex, Tileset};
use neoroll_world::map::element::Element as MapElement;

use crate::{graphics::TileName, plugins::map::element::Element};

pub const MAP_TILESET_NAME: &str = "Map";

pub fn element_tile_name(element: &MapElement) -> TileName {
    match element {
        MapElement::Tree => TileName("Tree".to_string()),
    }
}

#[derive(Resource, Default)]
pub struct MapTileset {
    pub handle: Option<Handle<Tileset>>,
}

pub fn spawn(
    atlas: &Handle<TextureAtlas>,
    tile_index: &TileIndex,
    point: Vec3,
    color: Color,
) -> (Element, SpriteSheetBundle) {
    (
        Element,
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
    )
}
