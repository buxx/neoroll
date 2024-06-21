use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{material::Material, Quantity};

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
