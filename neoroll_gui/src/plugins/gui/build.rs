use bevy::{prelude::*, render::view::RenderLayers};
use bevy_tileset::prelude::{TileIndex, Tilesets};
use neoroll_world::gameplay::build::Buildable;

use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    layer::LAYER_SCENE_ITEMS,
    plugins::world::tileset::WORLD_TILESET_NAME,
    utils::TileName,
};

use super::{Current, GuiState};

#[derive(Component)]
pub struct BuildCursor;


#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn display_build_cursor(
    state: Res<GuiState>,
    windows: Query<&Window>,
    mut cursor: Query<(&BuildCursor, &mut Transform)>,
    camera: Query<
        (&Camera, &GlobalTransform),
        (With<SceneItemsCamera>, Without<BackgroundCamera>),
    >,
) {
    if let Current::Build(_) = state.current() {
        let window = windows.single();
        if let Ok((_, mut transform)) = cursor.get_single_mut() {
            let (camera, camera_transform) = camera.single();
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                transform.translation = Vec3::new(world_position.x, world_position.y, 0.);
            }
        }
    }
}

pub fn spawn_display_cursor(mut commands: Commands, buildable: Buildable, tilesets: Tilesets) {
    if let Some(tileset) = tilesets.get_by_name(WORLD_TILESET_NAME) {
        let atlas = tileset.atlas();
        let (tile_index, _) = &tileset.select_tile(buildable.tile_name()).unwrap();
        commands.spawn((
            BuildCursor,
            match tile_index {
                // TODO: refactor with other match like this
                TileIndex::Standard(index) => {
                    let sprite = TextureAtlasSprite::new(*index);
                    SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(0., 0., 0.),
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
        ));
    }
}
