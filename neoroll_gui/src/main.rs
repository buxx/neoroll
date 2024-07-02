use bevy::prelude::*;
use bevy_egui::{egui, EguiPlugin};
use bevy_prototype_lyon::prelude::*;
use bevy_tileset::prelude::*;

use neoroll_server::{
    gateway::Gateways,
    server::{self, ClientMessage},
    state::game::ClientGameMessage,
};
use neoroll_world::gameplay::tribe::{Tribe, TribeId};
use plugins::{
    game::GameStatePlugin, gui::GuiPlugin, inputs::UserInputsPlugin, map::MapDisplayPlugin,
    server::ServerGatewayPlugin, world::WorldDisplayPlugin,
};
use setup::setup_;

mod camera;
mod debug;
mod graphics;
mod image;
mod layer;
mod plugins;
mod scene;
mod setup;
mod shortcut;
mod utils;

fn main() {
    // FIXME: When server behind network, gateways must be built server side and managed by network connections
    let mut gateways = Gateways::new();
    let gateway = gateways.register();
    server::spawn(gateways);

    // TODO: in game (player choose name, etc)
    gateway
        .send(ClientMessage::Game(ClientGameMessage::CreateTribe(
            Tribe::new(TribeId::new()),
        )))
        .unwrap();

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
