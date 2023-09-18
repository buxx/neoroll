use bevy::prelude::*;
use neoroll_world::{
    generator::{dummy::DummyWorldGenerator, WorldGenerator},
    map::builder::WorldMapBuilder,
};

use crate::{
    camera::PlayerCamera,
    graphics::tileset::{map::MapTileset, world::WorldTileset},
    plugins::{map::updater::MapUpdater, world::updater::WorldUpdater},
};

pub fn setup_(
    mut world_tileset: ResMut<WorldTileset>,
    mut map_tileset: ResMut<MapTileset>,
    mut world_reader: ResMut<WorldUpdater>,
    mut map_reader: ResMut<MapUpdater>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));

    world_tileset.handle = Some(asset_server.load("tilesets/world.ron"));
    map_tileset.handle = Some(asset_server.load("tilesets/map.ron"));

    // TODO : this part will be "server side" and network stuff
    let world = DummyWorldGenerator::new(500, 500).generate();
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
