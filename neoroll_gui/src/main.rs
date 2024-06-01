use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_tileset::prelude::*;

use neoroll_server::{
    gateway::Gateways,
    server::{self},
};
use plugins::{
    game::GameStatePlugin, gui::GuiPlugin, inputs::UserInputsPlugin, map::MapDisplayPlugin, server::ServerGatewayPlugin, world::WorldDisplayPlugin
};
use setup::setup_;

mod camera;
mod debug;
mod graphics;
mod layer;
mod plugins;
mod scene;
mod setup;

fn main() {
    // FIXME: When server behind network, gateways must be built server side and managed by network connections
    let mut gateways = Gateways::new();
    let gateway = gateways.register();
    server::spawn(gateways);

    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            ShapePlugin,
            TilesetPlugin::default(),
            UserInputsPlugin,
            ServerGatewayPlugin::new(gateway),
            WorldDisplayPlugin,
            MapDisplayPlugin,
            GameStatePlugin,
            GuiPlugin,
        ))
        .add_systems(Startup, setup_)
        .run();
}
