use std::collections::HashMap;

use neoroll_world::{
    entity::structure::Structure,
    gameplay::{
        build::{Buildable, TryBuildError},
        target::{ComputedTarget, Target, TargetId, WaitingReason},
        tribe::{structure::StructureOwn, Tribe, TribeId},
    },
    space::AbsoluteWorldPoint,
};
use settings::TribeSettings;

use crate::gateway::ClientId;

use super::client::ClientGameState;

pub mod settings;

#[derive(Default)]
pub struct GameState {
    tribes: HashMap<TribeId, Tribe>,
    client_tribe: HashMap<ClientId, TribeId>,
    structures_own: HashMap<TribeId, Vec<StructureOwn>>,
    client_speed_requests: HashMap<ClientId, u8>,
    tribe_settings: HashMap<TribeId, TribeSettings>,
    tribe_targets: HashMap<TribeId, Vec<ComputedTarget>>,
    tribe_waitings: HashMap<TribeId, HashMap<TargetId, Vec<WaitingReason>>>,
}

// FIXME BS NOW: need default value of speed for each clients (connected or not)
impl GameState {
    pub fn new_tribe(&mut self, tribe: Tribe) {
        let tribe_id = *tribe.id();

        self.tribes.insert(tribe_id, tribe);

        // TODO: Is that the good place for tribe init ?
        self.tribe_settings
            .insert(tribe_id, TribeSettings::default());
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
            owns.iter().for_each(|own| match &filter {
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
            .push(own);
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

    pub fn tribe_settings(&self) -> &HashMap<TribeId, TribeSettings> {
        &self.tribe_settings
    }

    pub fn tribe_settings_mut(&mut self) -> &mut HashMap<TribeId, TribeSettings> {
        &mut self.tribe_settings
    }

    pub fn tribe_waitings(&self) -> &HashMap<TribeId, HashMap<TargetId, Vec<WaitingReason>>> {
        &self.tribe_waitings
    }

    pub fn set_waitings(
        &mut self,
        tribe_id: &TribeId,
        target_id: &TargetId,
        waitings: Vec<WaitingReason>,
    ) {
        self.tribe_waitings
            .entry(*tribe_id)
            .or_default()
            .insert(*target_id, waitings);
    }

    pub fn tribe_targets(&self) -> &HashMap<TribeId, Vec<ComputedTarget>> {
        &self.tribe_targets
    }

    pub fn set_tribe_targets(&mut self, tribe_id: TribeId, value: Vec<ComputedTarget>) {
        self.tribe_targets.insert(tribe_id, value);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientGameMessage {
    CreateTribe(Tribe),
    TryBuild(Buildable, AbsoluteWorldPoint),
    RequestServerSpeed(u8),
    Target(TargetId, TargetMessage),
}

#[derive(Debug)]
pub enum GameChange {
    ImmediateClientGameStateRefresh(ClientId),
    SendClientGameState(ClientId, ClientGameState),
    ComputedTarget(TribeId, ComputedTargetChange),
    Waiting(TribeId, WaitingChange),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerGameMessage {
    TryBuildError(ClientGameMessage, TryBuildError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetMessage {
    Set(Target),
    New(Target),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComputedTargetChange {
    Set(Vec<ComputedTarget>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WaitingChange {
    Set(TargetId, Vec<WaitingReason>),
}
