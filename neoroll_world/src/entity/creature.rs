use std::fmt::Display;

use crate::{
    gameplay::{behavior::Behavior, job::Job, tribe::TribeId},
    space::AbsoluteWorldPoint,
};

use super::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Creature {
    id: CreatureId,
    tribe_id: TribeId,
    point: AbsoluteWorldPoint,
    job: Job,
    behavior: Behavior,
}

impl Creature {
    pub fn new(id: CreatureId, tribe_id: TribeId, position: AbsoluteWorldPoint) -> Self {
        Self {
            id,
            tribe_id,
            point: position,
            job: Default::default(),
            behavior: Default::default(),
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

    pub fn tribe_id(&self) -> &TribeId {
        &self.tribe_id
    }

    pub fn job(&self) -> &Job {
        &self.job
    }

    pub fn set_job(&mut self, job: Job) {
        self.job = job;
    }

    pub fn behavior(&self) -> &Behavior {
        &self.behavior
    }

    pub fn set_behavior(&mut self, behavior: Behavior) {
        self.behavior = behavior;
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

impl Display for CreatureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Debug)]
pub enum CreatureChange {
    New(Creature),
    SetPoint(AbsoluteWorldPoint),
    SetJob(Job),
    SetBehavior(Behavior),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PartialCreature {
    id: CreatureId,
    tribe_id: TribeId,
    point: AbsoluteWorldPoint,
    job: Job,
    behavior: Behavior,
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

    pub fn job(&self) -> &Job {
        &self.job
    }

    pub fn behavior(&self) -> &Behavior {
        &self.behavior
    }

    pub fn set_job(&mut self, job: Job) {
        self.job = job;
    }

    pub fn set_behavior(&mut self, behavior: Behavior) {
        self.behavior = behavior;
    }
}

impl From<Creature> for PartialCreature {
    fn from(value: Creature) -> Self {
        Self {
            id: *value.id(),
            tribe_id: *value.tribe_id(),
            point: *value.point(),
            job: value.job().clone(),
            behavior: value.behavior().clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartialCreatureChange {
    SetPoint(AbsoluteWorldPoint),
    SetJob(Job),
    SetBehavior(Behavior),
}
