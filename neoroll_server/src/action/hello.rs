use crate::{action::UpdateAction, state::StateChange};

use super::{Action, ActionChange, BodyTick, NextTick};

const TICK_FREQUENCY: u64 = 1;

#[derive(Debug, PartialEq)]
pub struct SayHello {
    counter: usize,
    value: u64,
}

impl SayHello {
    fn new() -> Self {
        Self {
            counter: 0,
            value: 0,
        }
    }
}

impl BodyTick<SayHelloChange> for SayHello {
    fn tick(
        &self,
        id: super::ActionId,
        state: &crate::state::State,
    ) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];

        if self.counter == 5 {
            changes.push(StateChange::Action(id, ActionChange::Remove))
        } else {
            // println!("Hello");
            let x: u64 = (1..=1000000).product();
            changes.extend(vec![
                StateChange::Action(
                    id,
                    ActionChange::Update(UpdateAction::SayHello(SayHelloChange::IncrementCounter)),
                ),
                StateChange::Action(
                    id,
                    ActionChange::Update(UpdateAction::SayHello(SayHelloChange::SetValue(x))),
                ),
            ])
        };

        (NextTick(*state.frame_i() + TICK_FREQUENCY), changes)
    }

    fn apply(&mut self, change: SayHelloChange) {
        match change {
            SayHelloChange::IncrementCounter => self.counter += 1,
            SayHelloChange::SetValue(value) => self.value = value,
        }
    }
}

pub enum SayHelloChange {
    IncrementCounter,
    SetValue(u64),
}

pub struct SayHelloActionBuilder;

impl SayHelloActionBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Action {
        Action::SayHello(SayHello::new())
    }
}
