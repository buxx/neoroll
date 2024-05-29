use crate::space::AbsoluteWorldPoint;

use super::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Creature {
    id: CreatureId,
    position: AbsoluteWorldPoint,
}
impl Creature {
    pub fn new(id: CreatureId, position: AbsoluteWorldPoint) -> Self {
        Self { id, position }
    }

    pub fn id(&self) -> &CreatureId {
        &self.id
    }

    pub fn position(&self) -> &AbsoluteWorldPoint {
        &self.position
    }

    pub fn set_position(&mut self, position: AbsoluteWorldPoint) {
        self.position = position;
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
