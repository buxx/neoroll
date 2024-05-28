use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

use crate::action::{Action, ActionChange, ActionId, NextTick};

pub struct State {
    frame_i: FrameI,
    actions: HashMap<ActionId, WrappedAction>,
}

impl State {
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
            };
        }
    }

    pub fn new() -> Self {
        Self {
            frame_i: FrameI(0),
            actions: HashMap::new(),
        }
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

    pub fn frame_i(&self) -> &FrameI {
        &self.frame_i
    }
}

pub enum StateChange {
    Action(ActionId, ActionChange),
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
