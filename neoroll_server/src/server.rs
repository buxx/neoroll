use std::{
    fs,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
};

use crate::{
    gateway::{ClientId, ClientMessageEnveloppe, Gateways},
    run::RunnerBuilder,
    state::{
        client::ClientGameState,
        game::{ClientGameMessage, GameState},
        State,
    },
    subscriptions::{Subscriptions, SubscriptionsMessage},
};
use neoroll_world::{
    entity::creature::{CreatureId, PartialCreatureChange},
    map::{area::MapArea, patch::NewSectors, Map},
    space::{area::WorldArea, patch::NewLayers, world::World},
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerMessageEnveloppe {
    Broadcast(ServerMessage),
    To(ClientId, ServerMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerMessage {
    Hello(Uuid),
    NewWorldLayers(WorldArea, NewLayers), // TODO: World(x)
    NewMapSectors(MapArea, NewSectors),   // TODO: Map(x)
    Creature(CreatureId, PartialCreatureChange),
    NewClientGameState(ClientGameState),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Hello(Uuid),
    RequireWorldArea(WorldArea, WorldArea), // (requested_area, ignore_area)
    RequireMapArea(MapArea, MapArea),       // (requested_area, ignore_area)
    Subscriptions(SubscriptionsMessage),
    Game(ClientGameMessage),
}

// TODO : this part will be "server side" and network stuff
//pub fn spawn(server_sender: Sender<ServerMessage>, client_receiver: Receiver<ClientMessage>) {
pub fn spawn(gateways: Gateways) {
    gateways.start();
    let gateways = Arc::new(RwLock::new(gateways));
    let subscriptions = Arc::new(RwLock::new(Subscriptions::new()));
    let world = bincode::deserialize::<World>(&fs::read("world.bin").unwrap()).unwrap();
    let map = bincode::deserialize::<Map>(&fs::read("map.bin").unwrap()).unwrap();
    let game = GameState::default();

    let world = Arc::new(RwLock::new(world));
    let map = Arc::new(RwLock::new(map));
    let game = Arc::new(RwLock::new(game));

    // TODO: separate code (other crate) ...
    let gateways_ = Arc::clone(&gateways);
    let subscriptions_ = Arc::clone(&subscriptions);
    let world_ = Arc::clone(&world);
    let map_ = Arc::clone(&map);
    let game_ = Arc::clone(&game);
    thread::spawn(move || Server::new(gateways_, subscriptions_, world_, map_, game_).run());

    // TODO: like in OpenCombat, permit remote (network) server instead embedded server
    thread::spawn(|| {
        RunnerBuilder::new(gateways, subscriptions)
            .actions(vec![])
            .build(State::new(world, map, game))
            .run();
    });
}

pub struct Server {
    gateways: Arc<RwLock<Gateways>>,
    subscriptions: Arc<RwLock<Subscriptions>>,
    world: Arc<RwLock<World>>, // NOTE: Server should only read world (Runner is only allowed to write)
    map: Arc<RwLock<Map>>, // NOTE: Server should only read map (Runner is only allowed to write)
    game: Arc<RwLock<GameState>>,
}

impl Server {
    pub fn new(
        gateways: Arc<RwLock<Gateways>>,
        subscriptions: Arc<RwLock<Subscriptions>>,
        world: Arc<RwLock<World>>,
        map: Arc<RwLock<Map>>,
        game: Arc<RwLock<GameState>>,
    ) -> Self {
        Self {
            gateways,
            subscriptions,
            world,
            map,
            game,
        }
    }

    fn subscriptions(&self) -> RwLockReadGuard<Subscriptions> {
        self.subscriptions.read().unwrap()
    }

    fn subscriptions_mut(&self) -> RwLockWriteGuard<Subscriptions> {
        self.subscriptions.write().unwrap()
    }

    fn game(&self) -> RwLockReadGuard<GameState> {
        self.game.read().unwrap()
    }

    fn game_mut(&self) -> RwLockWriteGuard<GameState> {
        self.game.write().unwrap()
    }

    pub fn run(&self) {
        while let Ok(message) = self.gateways.read().unwrap().receive() {
            self.react(message)
        }
    }

    pub fn react(&self, message: ClientMessageEnveloppe) {
        let ClientMessageEnveloppe(client_id, message) = message;

        // TODO: dispatch code into separated modules
        match message {
            ClientMessage::Hello(_) => {
                // TODO: Heartbeat
            }
            ClientMessage::RequireWorldArea(area, ignore_area) => {
                let new_layers =
                    NewLayers::from_world_area(&self.world.read().unwrap(), &area, &ignore_area);

                self.gateways
                    .read()
                    .unwrap()
                    .send(ServerMessageEnveloppe::To(
                        client_id,
                        ServerMessage::NewWorldLayers(area, new_layers),
                    ))
                    .unwrap();
            }
            ClientMessage::RequireMapArea(area, ignore_area) => {
                let new_sectors =
                    NewSectors::from_map_area(&self.map.read().unwrap(), &area, &ignore_area);
                self.gateways
                    .read()
                    .unwrap()
                    .send(ServerMessageEnveloppe::To(
                        client_id,
                        ServerMessage::NewMapSectors(area, new_sectors),
                    ))
                    .unwrap();
            }
            ClientMessage::Subscriptions(subscription) => match subscription {
                SubscriptionsMessage::SetCreatures(creatures) => {
                    self.subscriptions_mut().set_creatures(client_id, creatures)
                }
                SubscriptionsMessage::SetArea(area) => {
                    self.subscriptions_mut().set_area(client_id, area)
                }
            },
            ClientMessage::Game(message) => match message {
                ClientGameMessage::CreateTribe(tribe) => {
                    let mut game = self.game_mut();
                    game.set_client_tribe_id(client_id, *tribe.id());
                    game.new_tribe(tribe);
                }
            },
        }
    }
}
