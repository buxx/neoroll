use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{job::Job, material::Material, need::Need, Quantity};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    KeepStock(Material, TargetQuantity), // Quantity per habitant
}

/// All in game things have same unit
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum TargetQuantity {
    Fixed(Quantity),
    PerHuman(Quantity),
}

impl Target {
    pub fn name(&self) -> String {
        match self {
            Target::KeepStock(material, _) => format!("Keep stock of {}", &material.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetId(Uuid);

impl TargetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TargetId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedTarget {
    id: TargetId,
    target: Target,
    covered: bool,
    affected: usize,
    needs: Vec<Need>,
}

impl ComputedTarget {
    pub fn new(
        id: TargetId,
        target: Target,
        covered: bool,
        affected: usize,
        needs: Vec<Need>,
    ) -> Self {
        Self {
            id,
            target,
            covered,
            affected,
            needs,
        }
    }

    pub fn id(&self) -> &TargetId {
        &self.id
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn affected(&self) -> usize {
        self.affected
    }

    pub fn needs(&self) -> &Vec<Need> {
        &self.needs
    }

    pub fn covered(&self) -> bool {
        self.covered
    }

    pub fn state_string(&self) -> &str {
        if self.covered {
            return "Covered";
        }

        if self.affected != 0 {
            return "On Going";
        }

        "Waiting"
    }
}

impl From<&Target> for Job {
    fn from(value: &Target) -> Self {
        match value {
            Target::KeepStock(material, _) => {
                //
                match material {
                    Material::Resource(resource) => Job::SearchResource(*resource),
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WaitingReason {
    NotEnoughWorker,
    NotEnoughMaterial(Material),
}
