use crate::state::{client::builder::ClientGameStateBuilder, game::GameChange, State, StateChange};

use super::{ActionId, BodyTick, NextTick};

#[derive(Debug, PartialEq)]
pub struct ComputeAndSendClientStates;

impl BodyTick<()> for ComputeAndSendClientStates {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut messages = vec![];

        for client_id in state.game().client_ids() {
            let client_state = ClientGameStateBuilder::new(state).build(&client_id);
            messages.push(StateChange::Game(GameChange::SendClientGameState(
                client_id,
                client_state,
            )))
        }

        (NextTick(*state.frame_i() + 5), messages)
    }

    fn apply(&mut self, _change: ()) {}
}
