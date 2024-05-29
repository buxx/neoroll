use crossbeam::channel::{Receiver, Sender};

use bevy::ecs::system::Resource;
use neoroll_server::server::{ClientMessage, ServerMessage};

#[derive(Resource)]
pub struct Gateway {
    server_receiver: Receiver<ServerMessage>,
    client_sender: Sender<ClientMessage>,
}

impl Gateway {
    pub fn new(
        server_receiver: Receiver<ServerMessage>,
        client_sender: Sender<ClientMessage>,
    ) -> Self {
        Self {
            server_receiver,
            client_sender,
        }
    }

    pub fn send(&self, message: ClientMessage) {
        self.client_sender.send(message).unwrap();
    }

    pub fn read(&self) -> Vec<ServerMessage> {
        self.server_receiver.try_iter().collect()
    }
}
