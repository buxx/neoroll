use std::{
    fs,
    sync::{Arc, RwLock},
    thread,
};

use crate::{
    gateway::{ClientId, ClientMessageEnveloppe, Gateways},
    run::RunnerBuilder,
    state::State,
};
use neoroll_world::{
    map::{area::MapArea, patch::NewSectors, Map},
    space::{area::WorldArea, patch::NewLayers, world::World},
};

#[derive(Debug, Clone)]
pub enum ServerMessageEnveloppe {
    Broadcast(ServerMessage),
    To(ClientId, ServerMessage),
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    NewWorldLayers(WorldArea, NewLayers),
    NewMapSectors(MapArea, NewSectors),
}

#[derive(Debug, Clone)]
pub enum ClientMessage {
    RequireWorldArea(WorldArea, WorldArea), // (requested_area, ignore_area)
    RequireMapArea(MapArea, MapArea),       // (requested_area, ignore_area)
}

// TODO : this part will be "server side" and network stuff
//pub fn spawn(server_sender: Sender<ServerMessage>, client_receiver: Receiver<ClientMessage>) {
pub fn spawn(gateways: Gateways) {
    let world = bincode::deserialize::<World>(&fs::read("world.bin").unwrap()).unwrap();
    let map = bincode::deserialize::<Map>(&fs::read("map.bin").unwrap()).unwrap();

    let world = Arc::new(RwLock::new(world));
    let map = Arc::new(RwLock::new(map));

    gateways.start();

    // TODO: separate code (other crate) ...
    let world_ = Arc::clone(&world);
    let map_ = Arc::clone(&map);
    thread::spawn(move || {
        while let Ok(ClientMessageEnveloppe(client_id, message)) = gateways.receive() {
            match message {
                ClientMessage::RequireWorldArea(area, ignore_area) => {
                    let new_layers =
                        NewLayers::from_world_area(&world_.read().unwrap(), &area, &ignore_area);
                    gateways
                        .send(ServerMessageEnveloppe::To(
                            client_id,
                            ServerMessage::NewWorldLayers(area, new_layers),
                        ))
                        .unwrap();
                }
                ClientMessage::RequireMapArea(area, ignore_area) => {
                    let new_sectors =
                        NewSectors::from_map_area(&map_.read().unwrap(), &area, &ignore_area);
                    gateways
                        .send(ServerMessageEnveloppe::To(
                            client_id,
                            ServerMessage::NewMapSectors(area, new_sectors),
                        ))
                        .unwrap();
                }
            }
        }
    });
    // TODO: like in OpenCombat, permit remote server instead embedded server
    thread::spawn(|| {
        RunnerBuilder::new()
            .actions(vec![])
            .build(State::new(world, map))
            .run();
    });
}
