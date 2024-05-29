use crossbeam::channel::{Receiver, Sender};
use std::{fs, sync::{Arc, RwLock}, thread};

use neoroll_server::{run::RunnerBuilder, state::State};
use neoroll_world::{
    map::{area::MapArea, patch::NewSectors, Map},
    space::{area::WorldArea, patch::NewLayers, world::EntireWorld},
};

pub enum ServerMessage {
    NewWorldLayers(WorldArea, NewLayers),
    NewMapSectors(MapArea, NewSectors),
}
pub enum ClientMessage {
    RequireWorldArea(WorldArea, WorldArea), // (requested_area, ignore_area)
    RequireMapArea(MapArea, MapArea),       // (requested_area, ignore_area)
}

// TODO : this part will be "server side" and network stuff
pub fn spawn(server_sender: Sender<ServerMessage>, client_receiver: Receiver<ClientMessage>) {
    let world = bincode::deserialize::<EntireWorld>(&fs::read("world.bin").unwrap()).unwrap();
    let map = bincode::deserialize::<Map>(&fs::read("map.bin").unwrap()).unwrap();

    let world = Arc::new(RwLock::new(world));
    let map = Arc::new(RwLock::new(map));

    // TODO: separate code (other crate) ...
    let world_ = Arc::clone(&world);
    let map_ = Arc::clone(&map);
    thread::spawn(move || {
        while let Ok(message) = client_receiver.recv() {
            match message {
                ClientMessage::RequireWorldArea(area, ignore_area) => {
                    let new_layers = NewLayers::from_world_area(&world_.read().unwrap(), &area, &ignore_area);
                    server_sender
                        .send(ServerMessage::NewWorldLayers(area, new_layers))
                        .unwrap();
                }
                ClientMessage::RequireMapArea(area, ignore_area) => {
                    let new_sectors = NewSectors::from_map_area(&map_.read().unwrap(), &area, &ignore_area);
                    server_sender
                        .send(ServerMessage::NewMapSectors(area, new_sectors))
                        .unwrap();
                }
            }
        }
    });
    // TODO: like in OpenCombat, permit remote server instead embedded server
    thread::spawn(|| {
        RunnerBuilder::new().actions(vec![]).build(State::new(world, map)).run();
    });
}
