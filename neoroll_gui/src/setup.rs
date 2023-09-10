use bevy::prelude::*;

use crate::{camera::Camera, graphics::tileset::RegionTileset};

pub fn setup_(
    mut tileset: ResMut<RegionTileset>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), Camera));
    tileset.handle = Some(asset_server.load("tilesets/regions.ron"));
}
