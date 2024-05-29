use std::collections::HashMap;

use bevy::{
    asset::Handle,
    math::Vec3,
    render::{color::Color, view::RenderLayers},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    transform::components::Transform,
};
use bevy_tileset::prelude::TileIndex;
use neoroll_world::entity::creature::CreatureId;

use crate::layer::LAYER_SCENE_ITEMS;

use bevy::prelude::*;

#[derive(Component)]
pub struct CreatureComponent(pub CreatureId);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CreaturesMap(pub HashMap<CreatureId, Entity>);

pub fn spawn_creature(
    id: CreatureId,
    atlas: &Handle<TextureAtlas>,
    tile_index: &TileIndex,
    point: Vec3,
    color: Color,
) -> (CreatureComponent, SpriteSheetBundle, RenderLayers) {
    (
        CreatureComponent(id),
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
