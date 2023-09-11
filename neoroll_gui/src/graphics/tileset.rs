use bevy::prelude::*;
use bevy_tileset::prelude::*;

#[derive(Resource, Default)]
pub struct RegionTileset {
    pub handle: Option<Handle<Tileset>>,
}

fn spawn(atlas: &Handle<TextureAtlas>, tile_index: &TileIndex, point: Vec3) -> SpriteSheetBundle {
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
    }
}

pub fn display_world(tilesets: Tilesets, mut commands: Commands, mut has_ran: Local<bool>) {
    if *has_ran {
        return;
    }

    if let Some(tileset) = tilesets.get_by_name("Regions") {
        if let Some((tile_index, _)) = &tileset.select_tile("Grassland") {
            let atlas = tileset.atlas();
            for col in 0..10 {
                for row in 0..10 {
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
