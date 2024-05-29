use bevy::ecs::system::Resource;
use neoroll_server::{
    gateway::Gateway,
    server::{ClientMessage, ServerMessage},
};

#[derive(Resource)]
pub struct GatewayWrapper {
    gateway: Gateway,
}

impl GatewayWrapper {
    pub fn new(gateway: Gateway) -> Self {
        Self { gateway }
    }

    pub fn send(&self, message: ClientMessage) {
        self.gateway.send(message).unwrap();
    }

    pub fn read(&self) -> Vec<ServerMessage> {
        self.gateway.read()
    }
}
