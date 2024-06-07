use crate::{
    run::TICK_BASE_PERIOD,
    state::{client::builder::ClientGameStateBuilder, game::GameChange, State, StateChange},
};

use super::{ActionId, BodyTick, NextTick};

const TICK_PERIOD: u64 = TICK_BASE_PERIOD / 5;

#[derive(Debug, PartialEq)]
pub struct ComputeAndSendClientStates;

impl BodyTick<ComputeAndSendClientStatesChange> for ComputeAndSendClientStates {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut messages = vec![];

        for client_id in state.game().client_ids() {
            let client_state = ClientGameStateBuilder::new(state).build(&client_id);
            messages.push(StateChange::Game(GameChange::SendClientGameState(
                client_id,
                client_state,
            )))
        }

        (NextTick(*state.frame_i() + TICK_PERIOD), messages)
    }

    fn apply(&mut self, _change: ComputeAndSendClientStatesChange) {}
}

#[derive(Debug)]
pub enum ComputeAndSendClientStatesChange {}
