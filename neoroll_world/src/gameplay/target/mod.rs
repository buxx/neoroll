use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::structure::Structure;

use super::{
    job::Job,
    material::{Material, Resource},
    need::Need,
    Quantity,
};

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

impl Default for TargetQuantity {
    fn default() -> Self {
        Self::Fixed(Quantity(0))
    }
}

impl Target {
    pub fn name(&self) -> String {
        match self {
            Target::KeepStock(material, _) => format!("Keep stock of {}", &material.to_string()),
        }
    }

    pub fn default(&self) -> Target {
        match self {
            Target::KeepStock(material, _) => match material {
                Material::Resource(Resource::Food) => Target::KeepStock(
                    Material::Resource(Resource::Food),
                    TargetQuantity::PerHuman(Quantity(2000)),
                ),
                Material::Resource(Resource::RawFlint) => Target::KeepStock(
                    Material::Resource(Resource::RawFlint),
                    TargetQuantity::Fixed(Quantity(100)),
                ),
                Material::Resource(Resource::Branches) => Target::KeepStock(
                    Material::Resource(Resource::Branches),
                    TargetQuantity::Fixed(Quantity(5)),
                ),
            },
        }
    }

    pub fn is_same(&self, target: &Target) -> bool {
        match self {
            Target::KeepStock(material, _) => match target {
                Target::KeepStock(material_, _) => material.eq(material_),
            },
        }
    }

    pub fn carrying_enough_quantity(&self) -> Quantity {
        match self {
            Target::KeepStock(material, _) => match material {
                Material::Resource(Resource::Food) => Quantity(4000),
                Material::Resource(Resource::RawFlint) => Quantity(30),
                Material::Resource(Resource::Branches) => Quantity(1),
            },
        }
    }

    pub fn material(&self) -> Option<&Material> {
        match self {
            Target::KeepStock(material, _) => Some(material),
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
    priority: usize,
}

impl ComputedTarget {
    pub fn new(
        id: TargetId,
        target: Target,
        covered: bool,
        affected: usize,
        needs: Vec<Need>,
        priority: usize,
    ) -> Self {
        Self {
            id,
            target,
            covered,
            affected,
            needs,
            priority,
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

    pub fn priority(&self) -> usize {
        self.priority
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
    NeedOwnedStructure(Structure),
}

impl Display for WaitingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaitingReason::NotEnoughWorker => f.write_str("Worker"),
            WaitingReason::NotEnoughMaterial(material) => f.write_str(&material.to_string()),
            WaitingReason::NeedOwnedStructure(structure) => f.write_str(&structure.to_string()),
        }
    }
}
