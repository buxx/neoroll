use crate::gameplay::{material::Material, need::Need};

use super::TargetId;

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedNeed(pub TargetId, pub NeedState, pub Need);

#[derive(Debug, Clone, PartialEq)]
pub enum NeedState {
    Covered,
    Waiting(WaitingReason),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WaitingReason {
    NotEnoughMaterial(Material),
}

impl NeedState {
    pub fn is_satisfied(&self) -> bool {
        match self {
            NeedState::Covered => true,
            NeedState::Waiting(_) => false,
        }
    }
}
