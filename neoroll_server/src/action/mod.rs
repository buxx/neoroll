pub mod hello;
pub mod move_;
use move_::{MoveCreature, MoveCreatureChange};
use uuid::Uuid;

use crate::state::{FrameI, State, StateChange};

use self::hello::{SayHello, SayHelloChange};

#[derive(Debug, PartialEq)]
pub enum Action {
    SayHello(SayHello),
    MoveCreature(MoveCreature),
}

impl Action {
    pub fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        match self {
            Action::SayHello(body) => body.tick(id, state),
            Action::MoveCreature(body) => body.tick(id, state),
        }
    }

    pub fn apply(&mut self, change: UpdateAction) {
        match self {
            Action::SayHello(body) => {
                if let UpdateAction::SayHello(change) = change {
                    body.apply(change)
                }
            }
            Action::MoveCreature(body) => {
                if let UpdateAction::MoveCreature(change) = change {
                    body.apply(change)
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

pub enum ActionChange {
    New(Action),
    Update(UpdateAction),
    Remove,
    SetNextTick(NextTick),
}

pub enum UpdateAction {
    SayHello(SayHelloChange),
    MoveCreature(MoveCreatureChange),
}

pub trait BodyTick<T> {
    fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>);
    fn apply(&mut self, change: T);
}

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
