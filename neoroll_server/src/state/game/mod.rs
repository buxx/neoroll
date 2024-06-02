use std::collections::HashMap;

use neoroll_world::gameplay::tribe::{Tribe, TribeId};

use crate::gateway::ClientId;

use super::client::ClientGameState;

#[derive(Default)]
pub struct GameState {
    tribes: HashMap<TribeId, Tribe>,
    client_tribes: HashMap<ClientId, TribeId>,
}

impl GameState {
    pub fn new_tribe(&mut self, tribe: Tribe) {
        self.tribes.insert(*tribe.id(), tribe);
    }

    pub fn set_client_tribe_id(&mut self, client_id: ClientId, tribe_id: TribeId) {
        self.client_tribes.insert(client_id, tribe_id);
    }

    pub fn client_tribe_id(&self, client_id: &ClientId) -> Option<&TribeId> {
        self.client_tribes.get(client_id)
    }

    pub fn client_ids(&self) -> Vec<ClientId> {
        self.client_tribes.keys().map(|i| *i).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientGameMessage {
    CreateTribe(Tribe),
}

#[derive(Debug)]
pub enum GameChange {
    SendClientGameState(ClientId, ClientGameState),
}
