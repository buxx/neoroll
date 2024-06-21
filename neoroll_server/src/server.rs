use std::{
    fs,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
};

use crate::{
    action::{
        job::affect::AffectJobBuilder, need::ComputeTribeNeeds, Action, ActionChange, ActionId,
    },
    gateway::{ClientId, ClientMessageEnveloppe, Gateways},
    meta::MetaState,
    run::RunnerBuilder,
    state::{
        client::ClientGameState,
        game::{ClientGameMessage, GameChange, GameState, ServerGameMessage, TargetMessage},
        State, StateChange,
    },
    subscriptions::{Subscriptions, SubscriptionsMessage},
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use neoroll_world::{
    entity::creature::{CreatureId, PartialCreatureChange},
    gameplay::{build::TryBuild, tribe::structure::StructureOwn},
    map::{area::MapArea, patch::NewSectors, Map},
    space::{
        area::WorldArea,
        part::WorldPartMessage,
        patch::NewLayers,
        world::{StructureChange, World, WorldChange},
    },
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerMessageEnveloppe {
    Broadcast(ServerMessage),
    To(ClientId, ServerMessage),
}

// TODO: regroup in sub types
#[derive(Debug, Clone, PartialEq)]
pub enum ServerMessage {
    Hello(Uuid),
    NewWorldLayers(WorldArea, NewLayers), // TODO: World(x)
    NewMapSectors(MapArea, NewSectors),   // TODO: Map(x)
    Creature(CreatureId, PartialCreatureChange),
    NewClientGameState(ClientGameState),
    Game(ServerGameMessage),
    WorldPart(WorldPartMessage),
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
    let meta = Arc::new(RwLock::new(MetaState::default()));

    let (server_sender, server_receiver): (Sender<StateChange>, Receiver<StateChange>) =
        unbounded();
    // TODO: separate code (other crate) ...
    let gateways_ = Arc::clone(&gateways);
    let subscriptions_ = Arc::clone(&subscriptions);
    let world_ = Arc::clone(&world);
    let map_ = Arc::clone(&map);
    let game_ = Arc::clone(&game);
    thread::spawn(move || {
        Server::new(
            gateways_,
            subscriptions_,
            server_sender,
            world_,
            map_,
            game_,
        )
        .run()
    });

    // TODO: like in OpenCombat, permit remote (network) server instead embedded server
    thread::spawn(|| {
        RunnerBuilder::new(gateways, subscriptions, server_receiver)
            .actions(vec![])
            .build(State::new(world, map, game, meta))
            .run();
    });
}

pub struct Server {
    gateways: Arc<RwLock<Gateways>>,
    subscriptions: Arc<RwLock<Subscriptions>>,
    server_sender: Sender<StateChange>,
    world: Arc<RwLock<World>>, // NOTE: Server should only read world (Runner is only allowed to write)
    map: Arc<RwLock<Map>>, // NOTE: Server should only read map (Runner is only allowed to write)
    game: Arc<RwLock<GameState>>,
}

impl Server {
    pub fn new(
        gateways: Arc<RwLock<Gateways>>,
        subscriptions: Arc<RwLock<Subscriptions>>,
        server_sender: Sender<StateChange>,
        world: Arc<RwLock<World>>,
        map: Arc<RwLock<Map>>,
        game: Arc<RwLock<GameState>>,
    ) -> Self {
        Self {
            gateways,
            subscriptions,
            server_sender,
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

    fn send_to_client(&self, client_id: ClientId, message: ServerMessage) {
        self.gateways
            .read()
            .unwrap()
            .send(ServerMessageEnveloppe::To(client_id, message))
            .unwrap();
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
                self.send_to_client(client_id, ServerMessage::NewWorldLayers(area, new_layers));
            }
            ClientMessage::RequireMapArea(area, ignore_area) => {
                let new_sectors =
                    NewSectors::from_map_area(&self.map.read().unwrap(), &area, &ignore_area);
                self.send_to_client(client_id, ServerMessage::NewMapSectors(area, new_sectors));
            }
            ClientMessage::Subscriptions(subscription) => match subscription {
                SubscriptionsMessage::SetCreatures(creatures) => {
                    self.subscriptions_mut().set_creatures(client_id, creatures)
                }
                SubscriptionsMessage::SetArea(area) => {
                    self.subscriptions_mut().set_area(client_id, area)
                }
                SubscriptionsMessage::PushCreatures(creature_id) => {
                    self.subscriptions_mut()
                        .push_creature(client_id, creature_id);
                }
            },
            ClientMessage::Game(message) => match &message {
                ClientGameMessage::CreateTribe(tribe) => {
                    let mut game = self.game_mut();
                    game.set_client_tribe_id(client_id, *tribe.id());
                    game.new_tribe(tribe.clone());

                    // FIXME: more elegant way to create new tribe "actions"
                    let affect_jobs_action_id = ActionId::new();
                    self.server_sender
                        .send(StateChange::Action(
                            affect_jobs_action_id,
                            ActionChange::New(AffectJobBuilder::new(*tribe.id()).build()),
                        ))
                        .unwrap();

                    let compute_needs_action_id = ActionId::new();
                    self.server_sender
                        .send(StateChange::Action(
                            compute_needs_action_id,
                            ActionChange::New(Action::ComputeTribeNeeds(ComputeTribeNeeds::new(
                                *tribe.id(),
                            ))),
                        ))
                        .unwrap();
                }
                ClientGameMessage::TryBuild(buildable, point) => {
                    let game = self.game();
                    let tribe_id = game.client_tribe_id(&client_id).unwrap();
                    match TryBuild::new(&self.world.read().unwrap()).try_(buildable, point) {
                        Ok(_) => {
                            self.server_sender
                                .send(StateChange::World(WorldChange::Structure(
                                    *point,
                                    StructureChange::SetOwned(StructureOwn::new(
                                        (*buildable).into(),
                                        *tribe_id,
                                        *point,
                                    )),
                                )))
                                .unwrap();
                        }
                        Err(error) => self
                            .gateways
                            .read()
                            .unwrap()
                            .send(ServerMessageEnveloppe::To(
                                client_id,
                                ServerMessage::Game(ServerGameMessage::TryBuildError(
                                    message, error,
                                )),
                            ))
                            .unwrap(),
                    }
                }
                ClientGameMessage::RequestServerSpeed(speed) => {
                    self.game_mut()
                        .set_client_speed_request(client_id, *speed.min(&200).max(&1));
                }
                ClientGameMessage::Target(id, message) => match message {
                    TargetMessage::Set(target) => {
                        let game = self.game();
                        let tribe_id = *game.client_tribe_id(&client_id).unwrap();
                        // Required because read game as mut line after
                        drop(game);

                        self.game_mut()
                            .tribe_settings_mut()
                            .entry(tribe_id)
                            .or_default()
                            .targets_mut()
                            .insert(*id, target.clone());
                        self.server_sender
                            .send(StateChange::Game(
                                GameChange::ImmediateClientGameStateRefresh(client_id),
                            ))
                            .unwrap();
                    }
                },
            },
        }
    }
}
