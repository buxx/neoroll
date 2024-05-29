use std::collections::HashMap;

use neoroll_world::{
    entity::creature::CreatureId,
    space::{area::WorldArea, AbsoluteWorldPoint},
};

use crate::gateway::ClientId;

pub struct Subscriptions {
    areas: HashMap<ClientId, WorldArea>,
    creatures: HashMap<ClientId, Vec<CreatureId>>,
}

impl Subscriptions {
    pub fn new() -> Self {
        Self {
            areas: HashMap::new(),
            creatures: HashMap::new(),
        }
    }

    pub fn set_area(&mut self, client_id: ClientId, area: WorldArea) {
        self.areas.insert(client_id, area);
    }

    pub fn set_creatures(&mut self, client_id: ClientId, creature_ids: Vec<CreatureId>) {
        self.creatures.insert(client_id, creature_ids);
    }

    pub fn to_point(&self, point: &AbsoluteWorldPoint) -> Vec<ClientId> {
        self.areas
            .iter()
            .filter(|(_, area)| area.include(point))
            .map(|(client_id, _)| *client_id)
            .collect()
    }
}

impl Default for Subscriptions {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: think about remove subscriptions when client disconnect
#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionsMessage {
    SetArea(WorldArea),
    SetCreatures(Vec<CreatureId>),
}
