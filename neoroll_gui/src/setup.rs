use std::fs;

use bevy::render::view::visibility::RenderLayers;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use neoroll_world::space::world::EntireWorld;
use neoroll_world::{
    generator::{dummy::DummyWorldGenerator, WorldGenerator},
    map::builder::WorldMapBuilder,
};

use crate::plugins::map::tileset::MapResources;
use crate::plugins::world::tileset::WorldTileset;
use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    layer::{LAYER_BACKGROUND, LAYER_SCENE_ITEMS},
    plugins::{
        map::{background::Background, updater::MapUpdater},
        world::updater::WorldUpdater,
    },
};

// TODO: dispatch setup into plugins when world & map
// generation will be server side for real
pub fn setup_(
    mut world_tileset: ResMut<WorldTileset>,
    mut map_resources: ResMut<MapResources>,
    mut world_reader: ResMut<WorldUpdater>,
    mut map_reader: ResMut<MapUpdater>,
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
            ..default()
        },
        Background,
        RenderLayers::layer(LAYER_BACKGROUND),
    ));

    // TODO : this part will be "server side" and network stuff
    let world = bincode::deserialize::<EntireWorld>(&fs::read("world.bin").unwrap()).unwrap();
    let map = WorldMapBuilder::new(&world).build();
    info!(
        "Generated world: {} lines, {} columns, so {} tiles",
        world.lines(),
        world.columns(),
        world.lines() * world.columns()
    );
    // Here a socket to server
    world_reader.world = Some(world);
    // Here a socket to server
    map_reader.map = Some(map);
    // END of fake server part
}
