pub mod hello;
use uuid::Uuid;

use crate::state::{FrameI, State, StateChange};

use self::hello::{SayHello, SayHelloChange};

#[derive(Debug, PartialEq)]
pub enum Action {
    SayHello(SayHello),
}

impl Action {
    pub fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        match self {
            Self::SayHello(body) => body.tick(id, state),
        }
    }

    pub fn apply(&mut self, change: UpdateAction) {
        match (self, change) {
            (Self::SayHello(body), UpdateAction::SayHello(action_change)) => {
                body.apply(action_change)
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

pub enum ActionChange {
    New(Action),
    Update(UpdateAction),
    Remove,
    SetNextTick(NextTick),
}

pub enum UpdateAction {
    SayHello(SayHelloChange),
}

pub trait BodyTick<T> {
    fn tick(&self, id: ActionId, _state: &State) -> (NextTick, Vec<StateChange>);
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
