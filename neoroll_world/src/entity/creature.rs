use crate::space::AbsoluteWorldPoint;

use super::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Creature {
    id: CreatureId,
    point: AbsoluteWorldPoint,
}
impl Creature {
    pub fn new(id: CreatureId, position: AbsoluteWorldPoint) -> Self {
        Self {
            id,
            point: position,
        }
    }

    pub fn id(&self) -> &CreatureId {
        &self.id
    }

    pub fn point(&self) -> &AbsoluteWorldPoint {
        &self.point
    }

    pub fn set_point(&mut self, position: AbsoluteWorldPoint) {
        self.point = position;
    }
}

impl Entity for Creature {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreatureId(Uuid);

impl CreatureId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for CreatureId {
    fn default() -> Self {
        Self::new()
    }
}

pub enum CreatureChange {
    New(Creature),
    SetPoint(AbsoluteWorldPoint),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PartialCreature {
    id: CreatureId,
    point: AbsoluteWorldPoint,
}

impl PartialCreature {
    pub fn id(&self) -> &CreatureId {
        &self.id
    }

    pub fn point(&self) -> &AbsoluteWorldPoint {
        &self.point
    }

    pub fn set_point(&mut self, point: AbsoluteWorldPoint) {
        self.point = point;
    }
}

impl From<Creature> for PartialCreature {
    fn from(value: Creature) -> Self {
        Self {
            id: *value.id(),
            point: *value.point(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartialCreatureChange {
    SetPoint(AbsoluteWorldPoint),
}
