pub mod drop;
pub mod target;
use client::{ComputeAndSendClientStates, ComputeAndSendClientStatesChange};
use collect::{CollectChange, CollectResource};
use drop::{DropOff, DropOffChange};
use job::{
    affect::{AffectJob, AffectJobChange},
    realize::{RealizeJob, RealizeJobChange},
};
use migrant::{IncomingMigrant, IncomingMigrantChange};
use move_::{MoveRandomly, MoveRandomlyChange, MoveTo, MoveToChange};
use neoroll_world::space::world::WorldChange;
use target::{ComputeTargets, ComputeTargetsChange};
use uuid::Uuid;

use crate::state::{FrameI, State, StateChange};

use self::hello::{SayHello, SayHelloChange};

pub mod client;
pub mod collect;
pub mod hello;
pub mod job;
pub mod migrant;
pub mod move_;

#[derive(Debug, PartialEq)]
pub enum Action {
    SayHello(SayHello),
    MoveTo(MoveTo),
    MoveRandomly(MoveRandomly),
    ComputeTargets(ComputeTargets),
    ComputeAndSendClientStates(ComputeAndSendClientStates),
    IncomingMigrant(IncomingMigrant),
    AffectJob(AffectJob),
    RealizeJob(RealizeJob),
    Collect(CollectResource),
    DropOff(DropOff),
}

impl Action {
    pub fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        match self {
            Action::SayHello(body) => body.tick(id, state),
            Action::MoveTo(body) => body.tick(id, state),
            Action::MoveRandomly(body) => body.tick(id, state),
            Action::ComputeTargets(body) => body.tick(id, state),
            Action::ComputeAndSendClientStates(body) => body.tick(id, state),
            Action::IncomingMigrant(body) => body.tick(id, state),
            Action::AffectJob(body) => body.tick(id, state),
            Action::RealizeJob(body) => body.tick(id, state),
            Action::Collect(body) => body.tick(id, state),
            Action::DropOff(body) => body.tick(id, state),
        }
    }

    pub fn stamp(&self) -> Vec<WorldChange> {
        match self {
            Action::SayHello(body) => body.stamp(),
            Action::MoveTo(body) => body.stamp(),
            Action::MoveRandomly(body) => body.stamp(),
            Action::ComputeTargets(body) => body.stamp(),
            Action::ComputeAndSendClientStates(body) => body.stamp(),
            Action::IncomingMigrant(body) => body.stamp(),
            Action::AffectJob(body) => body.stamp(),
            Action::RealizeJob(body) => body.stamp(),
            Action::Collect(body) => body.stamp(),
            Action::DropOff(body) => body.stamp(),
        }
    }

    pub fn take_off(&self) -> Vec<WorldChange> {
        match self {
            Action::SayHello(body) => body.take_off(),
            Action::MoveTo(body) => body.take_off(),
            Action::MoveRandomly(body) => body.take_off(),
            Action::ComputeTargets(body) => body.take_off(),
            Action::ComputeAndSendClientStates(body) => body.take_off(),
            Action::IncomingMigrant(body) => body.take_off(),
            Action::AffectJob(body) => body.take_off(),
            Action::RealizeJob(body) => body.take_off(),
            Action::Collect(body) => body.take_off(),
            Action::DropOff(body) => body.take_off(),
        }
    }

    pub fn apply(&mut self, change: UpdateAction) {
        match self {
            Action::SayHello(body) => {
                if let UpdateAction::SayHello(change) = change {
                    body.apply(change)
                }
            }
            Action::MoveTo(body) => {
                if let UpdateAction::MoveTo(change) = change {
                    body.apply(change)
                }
            }
            Action::MoveRandomly(body) => {
                if let UpdateAction::MoveRandomly(change) = change {
                    body.apply(change)
                }
            }
            Action::ComputeTargets(body) => {
                if let UpdateAction::ComputeTargets(change) = change {
                    body.apply(change)
                }
            }
            Action::ComputeAndSendClientStates(body) => {
                if let UpdateAction::ComputeAndSendClientStates(change) = change {
                    body.apply(change)
                }
            }
            Action::IncomingMigrant(body) => {
                if let UpdateAction::IncomingMigrant(change) = change {
                    body.apply(change)
                }
            }
            Action::AffectJob(body) => {
                if let UpdateAction::AffectJob(change) = change {
                    body.apply(change)
                }
            }
            Action::RealizeJob(body) => {
                if let UpdateAction::RealizeJob(change) = change {
                    body.apply(change)
                }
            }
            Action::Collect(body) => {
                if let UpdateAction::Collect(change) = change {
                    body.apply(change)
                }
            }
            Action::DropOff(body) => {
                if let UpdateAction::DropOff(change) = change {
                    body.apply(change)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionId(Uuid);

impl ActionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ActionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum ActionChange {
    New(Action),
    Update(UpdateAction),
    Remove,
    SetNextTick(NextTick),
}

#[derive(Debug)]
pub enum UpdateAction {
    SayHello(SayHelloChange),
    MoveTo(MoveToChange),
    MoveRandomly(MoveRandomlyChange),
    ComputeTargets(ComputeTargetsChange),
    ComputeAndSendClientStates(ComputeAndSendClientStatesChange),
    IncomingMigrant(IncomingMigrantChange),
    AffectJob(AffectJobChange),
    RealizeJob(RealizeJobChange),
    Collect(CollectChange),
    DropOff(DropOffChange),
}

// TODO: move T into Self::Type
pub trait BodyTick<T> {
    /// When action is added to sate, return here one shot world changes to apply
    fn stamp(&self) -> Vec<WorldChange> {
        vec![]
    }
    /// When action is added removed from sate, return here one shot world changes to apply
    fn take_off(&self) -> Vec<WorldChange> {
        vec![]
    }
    fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>);
    fn apply(&mut self, change: T);
}

#[derive(Debug)]
pub struct NextTick(FrameI);

impl NextTick {
    pub fn new(frame_id: FrameI) -> Self {
        Self(frame_id)
    }
}

impl PartialEq<FrameI> for NextTick {
    fn eq(&self, other: &FrameI) -> bool {
        self.0 == *other
    }
}
