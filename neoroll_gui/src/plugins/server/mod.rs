pub mod listener;

use bevy::prelude::*;
use gateway::GatewayWrapper;
use neoroll_server::gateway::Gateway;

pub mod gateway;

pub struct ServerGatewayPlugin {
    gateway: Gateway,
}

impl ServerGatewayPlugin {
    pub fn new(gateway: Gateway) -> Self {
        Self { gateway }
    }
}

impl Plugin for ServerGatewayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GatewayWrapper::new(self.gateway.clone()))
            .add_systems(Update, (listener::listen,));
    }
}
