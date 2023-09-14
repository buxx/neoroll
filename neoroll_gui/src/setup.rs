use bevy::prelude::*;
use neoroll_world::state::EntireWorld;

use crate::{
    camera::PlayerCamera, graphics::tileset::RegionTileset, plugins::world::updater::WorldUpdater,
};

pub fn setup_(
    mut tileset: ResMut<RegionTileset>,
    mut world_reader: ResMut<WorldUpdater>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
    tileset.handle = Some(asset_server.load("tilesets/regions.ron"));

    // TODO : this part will be "server side" and network stuff
    let entire_world = EntireWorld::from_random(100, 100);
    info!(
        "Generated world: {} lines, {} columns, {} tiles",
        entire_world.lines(),
        entire_world.columns(),
        entire_world.regions().len()
    );
    world_reader.world = Some(entire_world);
}
