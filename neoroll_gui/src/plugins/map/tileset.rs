use bevy::{prelude::*, render::view::RenderLayers};

use bevy_tileset::prelude::{TileIndex, Tileset};
use neoroll_world::map::element::Element as MapElement;

use crate::{graphics::TileName, layer::LAYER_SCENE_ITEMS, plugins::map::element::Element};

pub const MAP_TILESET_NAME: &str = "Map";

pub fn element_tile_name(element: &MapElement) -> TileName {
    match element {
        MapElement::Tree1a => TileName("Tree1a".to_string()),
        MapElement::Tree1b => TileName("Tree1b".to_string()),
        MapElement::Tree1c => TileName("Tree1c".to_string()),
        MapElement::Tree2a => TileName("Tree2a".to_string()),
        MapElement::Tree2b => TileName("Tree2b".to_string()),
        MapElement::Tree3a => TileName("Tree3a".to_string()),
        MapElement::Tree3b => TileName("Tree3b".to_string()),
        MapElement::Tree3c => TileName("Tree3c".to_string()),
        MapElement::Tree4a => TileName("Tree4a".to_string()),
        MapElement::Tree4b => TileName("Tree4b".to_string()),
        MapElement::Tree4c => TileName("Tree4c".to_string()),
        MapElement::Tree4d => TileName("Tree4d".to_string()),
    }
}

#[derive(Resource, Default)]
pub struct MapResources {
    pub tileset: Option<Handle<Tileset>>,
    pub background: Option<Handle<Image>>,
}

pub fn spawn(
    atlas: &Handle<TextureAtlas>,
    tile_index: &TileIndex,
    point: Vec3,
    color: Color,
) -> (Element, SpriteSheetBundle, RenderLayers) {
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
        RenderLayers::layer(LAYER_SCENE_ITEMS),
    )
}
