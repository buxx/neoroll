pub mod client;
pub mod game;
pub mod world;
use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use client::builder::ClientGameStateBuilder;
use game::{ComputedTargetChange, GameChange, GameState, WaitingChange};
use neoroll_world::{
    map::Map,
    space::world::{World, WorldChange},
};
use world::WorldModifier;

use crate::{
    action::{Action, ActionChange, ActionId, NextTick},
    gateway::Gateways,
    meta::MetaState,
    server::{ServerMessage, ServerMessageEnveloppe},
    subscriptions::Subscriptions,
};

pub struct State {
    frame_i: FrameI,
    actions: HashMap<ActionId, WrappedAction>,
    world: Arc<RwLock<World>>,
    map: Arc<RwLock<Map>>,
    game: Arc<RwLock<GameState>>,
    meta: Arc<RwLock<MetaState>>,
}

impl State {
    pub fn new(
        world: Arc<RwLock<World>>,
        map: Arc<RwLock<Map>>,
        game: Arc<RwLock<GameState>>,
        meta: Arc<RwLock<MetaState>>,
    ) -> Self {
        Self {
            frame_i: FrameI(0),
            actions: HashMap::new(),
            world,
            map,
            game,
            meta,
        }
    }

    pub fn frame_i(&self) -> &FrameI {
        &self.frame_i
    }

    pub fn world(&self) -> RwLockReadGuard<World> {
        self.world.read().unwrap()
    }

    fn world_mut(&self) -> RwLockWriteGuard<World> {
        self.world.write().unwrap()
    }

    pub fn map(&self) -> RwLockReadGuard<Map> {
        self.map.read().unwrap()
    }

    pub fn game(&self) -> RwLockReadGuard<GameState> {
        self.game.read().unwrap()
    }

    fn game_mut(&self) -> RwLockWriteGuard<GameState> {
        self.game.write().unwrap()
    }

    pub fn meta(&self) -> RwLockReadGuard<MetaState> {
        self.meta.read().unwrap()
    }

    pub fn meta_mut(&self) -> RwLockWriteGuard<MetaState> {
        self.meta.write().unwrap()
    }

    pub fn actions(&self) -> &HashMap<ActionId, WrappedAction> {
        &self.actions
    }

    /// Return actions to tick for current state
    pub fn to_do(&self) -> impl Iterator<Item = (&ActionId, &Action)> {
        self.actions
            .iter()
            .filter(|(_, w)| w.0 == self.frame_i)
            .map(|(id, w)| (id, &w.1))
    }

    pub fn increment(&mut self) {
        self.frame_i += FrameI(1);
        self.meta_mut().clear();
    }

    pub fn apply(
        &mut self,
        gateways: &Arc<RwLock<Gateways>>,
        subscriptions: &Arc<RwLock<Subscriptions>>,
        changes: Vec<StateChange>,
    ) {
        for change in changes {
            match change {
                StateChange::Action(id, ActionChange::New(action)) => {
                    let next = NextTick::new(self.frame_i + 1);
                    for change in action.stamp() {
                        WorldModifier::new(
                            gateways,
                            subscriptions,
                            &mut self.world_mut(),
                            &mut self.game_mut(),
                        )
                        .apply(change);
                    }

                    self.actions.insert(id, WrappedAction(next, action));
                }
                StateChange::Action(id, ActionChange::SetNextTick(next)) => {
                    self.actions.get_mut(&id).unwrap().0 = next;
                }
                StateChange::Action(id, ActionChange::Update(change)) => {
                    self.actions.get_mut(&id).unwrap().1.apply(change);
                }
                StateChange::Action(id, ActionChange::Remove) => {
                    if let Some(action) = self.actions.get(&id) {
                        for change in action.1.take_off() {
                            WorldModifier::new(
                                gateways,
                                subscriptions,
                                &mut self.world_mut(),
                                &mut self.game_mut(),
                            )
                            .apply(change);
                        }
                    }

                    self.actions.remove(&id);
                }
                StateChange::World(change) => {
                    WorldModifier::new(
                        gateways,
                        subscriptions,
                        &mut self.world_mut(),
                        &mut self.game_mut(),
                    )
                    .apply(change);
                }
                StateChange::Game(change) => match change {
                    GameChange::SendClientGameState(client_id, state) => {
                        gateways
                            .read()
                            .unwrap()
                            .send(ServerMessageEnveloppe::To(
                                client_id,
                                ServerMessage::NewClientGameState(state),
                            ))
                            .unwrap();
                    }
                    GameChange::ImmediateClientGameStateRefresh(client_id) => {
                        let client_state = ClientGameStateBuilder::new(self).build(&client_id);
                        gateways
                            .read()
                            .unwrap()
                            .send(ServerMessageEnveloppe::To(
                                client_id,
                                ServerMessage::NewClientGameState(client_state),
                            ))
                            .unwrap();
                    }
                    GameChange::ComputedTarget(tribe_id, change) => {
                        match change {
                            ComputedTargetChange::Set(targets) => {
                                self.game_mut().set_tribe_targets(tribe_id, targets);
                            }
                        };
                    }
                    GameChange::Waiting(tribe_id, change) => {
                        match change {
                            WaitingChange::Set(target_id, waitings) => {
                                self.game_mut()
                                    .set_waitings(&tribe_id, &target_id, waitings);
                            }
                        };
                    }
                },
            };
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            frame_i: FrameI(0),
            actions: Default::default(),
            world: Default::default(),
            map: Default::default(),
            game: Default::default(),
            meta: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum StateChange {
    Action(ActionId, ActionChange),
    World(WorldChange),
    Game(GameChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameI(pub u64);

impl Add<u64> for FrameI {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign for FrameI {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

struct WrappedAction(NextTick, Action);
