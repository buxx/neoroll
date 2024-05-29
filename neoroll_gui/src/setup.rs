use std::fs;

use bevy::render::view::visibility::RenderLayers;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use neoroll_world::entity::ground::Ground;
use neoroll_world::map::builder::MapBuilder;
use neoroll_world::map::Map;
use neoroll_world::space::world::World;

use crate::debug::WorldToTxt;
use crate::plugins::map::tileset::MapResources;
use crate::plugins::world::tileset::WorldTileset;
use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    layer::{LAYER_BACKGROUND, LAYER_SCENE_ITEMS},
    plugins::map::background::Background,
};

// TODO: dispatch setup into plugins when world & map
// generation will be server side for real
pub fn setup_(
    mut world_tileset: ResMut<WorldTileset>,
    mut map_resources: ResMut<MapResources>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        BackgroundCamera,
        RenderLayers::from_layers(&[LAYER_BACKGROUND]),
    ));
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        SceneItemsCamera,
        RenderLayers::from_layers(&[LAYER_SCENE_ITEMS]),
    ));

    world_tileset.handle = Some(asset_server.load("tilesets/world.ron"));
    map_resources.tileset = Some(asset_server.load("tilesets/map.ron"));
    let background = asset_server.load("map/background.png");
    map_resources.background = Some(background.clone());

    commands.spawn((
        SpriteBundle {
            texture: background,
            visibility: Visibility::Hidden,
            ..default()
        },
        Background,
        RenderLayers::layer(LAYER_BACKGROUND),
    ));
}
