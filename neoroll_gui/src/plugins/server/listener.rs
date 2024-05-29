use bevy::{
    ecs::{
        event::EventWriter,
        system::{Res, ResMut},
    },
    log::info,
};

use crate::{
    plugins::{
        map::container::{MapPartContainer, MapPartContainerRefreshed},
        world::container::{WorldPartContainer, WorldPartContainerRefreshed},
    },
    server::ServerMessage,
};

use super::gateway::Gateway as ServerGateway;

pub fn listen(
    server_gateway: Res<ServerGateway>,
    mut world_container_refreshed: EventWriter<WorldPartContainerRefreshed>,
    mut world_part: ResMut<WorldPartContainer>,
    mut map_container_refreshed: EventWriter<MapPartContainerRefreshed>,
    mut map_part: ResMut<MapPartContainer>,
) {
    for message in server_gateway.read() {
        match message {
            ServerMessage::NewWorldLayers(area, layers) => {
                info!("Received {} tiles", layers.len());
                world_part.0.switch(layers, area);
                world_container_refreshed.send(WorldPartContainerRefreshed);
            }
            ServerMessage::NewMapSectors(area, sectors) => {
                info!("Received {} sectors", sectors.len());
                // FIXME BS NOW: hardcoded lakes for now
                map_part.0.switch(sectors, vec![].clone(), area);
                map_container_refreshed.send(MapPartContainerRefreshed);
            }
        }
    }
}
