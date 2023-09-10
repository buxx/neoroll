use bevy::prelude::*;

use crate::{camera::CameraController, graphics::tileset::RegionTileset};

pub fn setup_(
    mut tileset: ResMut<RegionTileset>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), CameraController));
    tileset.handle = Some(asset_server.load("tilesets/regions.ron"));
}
