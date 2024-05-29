use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tileset::prelude::*;

use crossbeam::channel::{unbounded, Receiver, Sender};
use plugins::{
    inputs::UserInputsPlugin, map::MapDisplayPlugin, server::ServerGatewayPlugin,
    world::WorldDisplayPlugin,
};
use server::{ClientMessage, ServerMessage};
use setup::setup_;

mod camera;
mod debug;
mod graphics;
mod layer;
mod plugins;
mod scene;
mod server;
mod setup;

fn main() {
    // FIXME: channel should be able to manage different clients (with different needs ...)
    let (server_sender, server_receiver): (Sender<ServerMessage>, Receiver<ServerMessage>) =
        unbounded();
    let (client_sender, client_receiver): (Sender<ClientMessage>, Receiver<ClientMessage>) =
        unbounded();

    server::spawn(server_sender, client_receiver);

    App::new()
        .add_plugins((
            DefaultPlugins,
            ShapePlugin,
            TilesetPlugin::default(),
            UserInputsPlugin,
            ServerGatewayPlugin::new(server_receiver, client_sender),
            WorldDisplayPlugin,
            MapDisplayPlugin,
        ))
        .add_systems(Startup, setup_)
        .run();
}
