pub mod listener;
use crossbeam::channel::{Receiver, Sender};

use bevy::prelude::*;
use gateway::Gateway;
use neoroll_server::server::{ClientMessage, ServerMessage};

pub mod gateway;

pub struct ServerGatewayPlugin {
    server_receiver: Receiver<ServerMessage>,
    client_sender: Sender<ClientMessage>,
}

impl ServerGatewayPlugin {
    pub fn new(
        server_receiver: Receiver<ServerMessage>,
        client_sender: Sender<ClientMessage>,
    ) -> Self {
        Self {
            server_receiver,
            client_sender,
        }
    }
}

impl Plugin for ServerGatewayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gateway::new(
            self.server_receiver.clone(),
            self.client_sender.clone(),
        ))
        .add_systems(Update, (listener::listen,));
    }
}
