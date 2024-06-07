use std::collections::HashMap;

use neoroll_world::{
    entity::structure::Structure,
    gameplay::{
        build::{Buildable, TryBuildError},
        tribe::{structure::StructureOwn, Tribe, TribeId},
    },
    space::AbsoluteWorldPoint,
};

use crate::gateway::ClientId;

use super::client::ClientGameState;

#[derive(Default)]
pub struct GameState {
    tribes: HashMap<TribeId, Tribe>,
    client_tribe: HashMap<ClientId, TribeId>,
    structures_own: HashMap<TribeId, HashMap<AbsoluteWorldPoint, StructureOwn>>,
    client_speed_requests: HashMap<ClientId, u8>,
}

impl GameState {
    pub fn new_tribe(&mut self, tribe: Tribe) {
        self.tribes.insert(*tribe.id(), tribe);
    }

    pub fn set_client_tribe_id(&mut self, client_id: ClientId, tribe_id: TribeId) {
        self.client_tribe.insert(client_id, tribe_id);
    }

    pub fn client_tribe_id(&self, client_id: &ClientId) -> Option<&TribeId> {
        self.client_tribe.get(client_id)
    }

    pub fn client_ids(&self) -> Vec<ClientId> {
        self.client_tribe.keys().copied().collect()
    }

    pub fn tribe_ids(&self) -> Vec<TribeId> {
        self.tribes.keys().copied().collect()
    }

    pub fn tribe_structures(
        &self,
        tribe_id: &TribeId,
        filter: Option<Structure>,
    ) -> Vec<&StructureOwn> {
        let mut structures = vec![];
        if let Some(owns) = self.structures_own.get(tribe_id) {
            owns.iter().for_each(|(_, own)| match &filter {
                Some(type_) => {
                    if type_ == own.type_() {
                        structures.push(own)
                    }
                }
                None => structures.push(own),
            });
        }

        structures
    }

    pub fn set_structure_own(&mut self, own: StructureOwn) {
        self.structures_own
            .entry(*own.tribe_id())
            .or_default()
            .insert(*own.point(), own);
    }

    pub fn client_speed_requests(&self) -> &HashMap<ClientId, u8> {
        &self.client_speed_requests
    }

    pub fn set_client_speed_request(&mut self, client_id: ClientId, speed: u8) {
        self.client_speed_requests.insert(client_id, speed);
    }

    pub fn speed(&self) -> u64 {
        let values = self
            .client_speed_requests()
            .values()
            .map(|v| *v as u64)
            .collect::<Vec<u64>>();

        if !values.is_empty() {
            values.iter().sum::<u64>() / values.len() as u64
        } else {
            1
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientGameMessage {
    CreateTribe(Tribe),
    TryBuild(Buildable, AbsoluteWorldPoint),
    RequestServerSpeed(u8),
}

#[derive(Debug)]
pub enum GameChange {
    SendClientGameState(ClientId, ClientGameState),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerGameMessage {
    TryBuildError(ClientGameMessage, TryBuildError),
}
