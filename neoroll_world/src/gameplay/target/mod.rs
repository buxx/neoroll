pub mod need;
use need::WaitingReason;
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

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedTarget {
    id: TargetId,
    target: Target,
    state: TargetState,
    affected: usize,
}

impl ComputedTarget {
    pub fn new(id: TargetId, target: Target, state: TargetState, affected: usize) -> Self {
        Self {
            id,
            target,
            state,
            affected,
        }
    }

    pub fn id(&self) -> &TargetId {
        &self.id
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn state(&self) -> &TargetState {
        &self.state
    }

    pub fn affected(&self) -> usize {
        self.affected
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetState {
    Covered,
    InProgress(Vec<WaitingReason>),
    Waiting(Vec<WaitingReason>),
}

impl TargetState {
    pub fn is_satisfied(&self) -> bool {
        match self {
            TargetState::Covered => true,
            TargetState::InProgress(_) | TargetState::Waiting(_) => false,
        }
    }
}
