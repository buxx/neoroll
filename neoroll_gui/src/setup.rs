use bevy::prelude::*;
use neoroll_world::state::World;

use crate::{camera::PlayerCamera, graphics::tileset::RegionTileset, world::WorldContainer};

pub fn setup_(
    mut tileset: ResMut<RegionTileset>,
    mut world: ResMut<WorldContainer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
    tileset.handle = Some(asset_server.load("tilesets/regions.ron"));
    world.0 = World::from_random(100, 100);
}
