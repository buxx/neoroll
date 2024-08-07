use std::fmt::Display;

use crate::{
    gameplay::{behavior::Behavior, job::Job, material::Material, tribe::TribeId, Quantity},
    space::AbsoluteWorldPoint,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const MAX_CARRYING_QUANTITY: Quantity = Quantity(50000);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Creature {
    id: CreatureId,
    tribe_id: TribeId,
    point: AbsoluteWorldPoint,
    job: Job,
    behavior: Behavior,
    carrying: Vec<(Material, Quantity)>,
}

impl Creature {
    pub fn new(id: CreatureId, tribe_id: TribeId, position: AbsoluteWorldPoint) -> Self {
        Self {
            id,
            tribe_id,
            point: position,
            job: Default::default(),
            behavior: Default::default(),
            carrying: vec![],
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

    pub fn add_to_carrying(&mut self, material: Material, quantity: Quantity) {
        if let Some(quantity_) = self
            .carrying
            .iter_mut()
            .filter(|(m, _)| m == &material)
            .map(|(_, q)| q)
            .next()
        {
            *quantity_ += quantity;
        } else {
            self.carrying.push((material, quantity))
        }
    }

    pub fn remove_from_carrying(&mut self, material: Material, quantity: Quantity) {
        if let Some(quantity_) = self
            .carrying
            .iter_mut()
            .filter(|(m, _)| m == &material)
            .map(|(_, q)| q)
            .next()
        {
            quantity_.0 -= quantity.0.min(quantity_.0);
        }
    }

    pub fn cant_carry_more(&self) -> bool {
        self.carrying
            .iter()
            .map(|(_, q)| q.clone())
            .sum::<Quantity>()
            .0
            >= MAX_CARRYING_QUANTITY.0
    }

    pub fn carrying(&self) -> &[(Material, Quantity)] {
        &self.carrying
    }

    pub fn carrying_quantity(&self, filter: Option<Material>) -> Quantity {
        match &filter {
            Some(material) => self
                .carrying()
                .iter()
                .filter(|(m, _)| m == material)
                .map(|(_, q)| q.clone())
                .sum(),
            None => self.carrying().iter().map(|(_, q)| q.clone()).sum(),
        }
    }
}

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
    AddToCarrying(Material, Quantity),
    RemoveFromCarrying(Material, Quantity),
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
