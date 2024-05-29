use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use neoroll_world::{
    map::Map,
    space::world::{World, WorldChange},
};

use crate::action::{Action, ActionChange, ActionId, NextTick};

pub struct State {
    frame_i: FrameI,
    actions: HashMap<ActionId, WrappedAction>,
    world: Arc<RwLock<World>>,
    map: Arc<RwLock<Map>>,
}

impl State {
    pub fn new(world: Arc<RwLock<World>>, map: Arc<RwLock<Map>>) -> Self {
        Self {
            frame_i: FrameI(0),
            actions: HashMap::new(),
            world,
            map,
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

    fn map_mut(&self) -> RwLockWriteGuard<Map> {
        self.map.write().unwrap()
    }

    /// Return actions to tick for current state
    pub fn actions(&self) -> impl Iterator<Item = (&ActionId, &Action)> {
        self.actions
            .iter()
            .filter(|(_, w)| w.0 == self.frame_i)
            .map(|(id, w)| (id, &w.1))
    }

    pub fn increment(&mut self) {
        self.frame_i += FrameI(1);
    }

    pub fn apply(&mut self, changes: Vec<StateChange>) {
        for change in changes {
            match change {
                StateChange::Action(id, ActionChange::New(action)) => {
                    let next = NextTick::new(self.frame_i + 1);
                    self.actions.insert(id, WrappedAction(next, action));
                }
                StateChange::Action(id, ActionChange::SetNextTick(next)) => {
                    self.actions.get_mut(&id).unwrap().0 = next;
                }
                StateChange::Action(id, ActionChange::Update(change)) => {
                    self.actions.get_mut(&id).unwrap().1.apply(change);
                }
                StateChange::Action(id, ActionChange::Remove) => {
                    self.actions.remove(&id);
                }
                StateChange::World(change) => {
                    self.world_mut().apply(change);
                }
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
        }
    }
}

pub enum StateChange {
    Action(ActionId, ActionChange),
    World(WorldChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameI(u64);

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
