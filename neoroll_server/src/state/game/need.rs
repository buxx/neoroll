use neoroll_world::gameplay::{need::Need, target::TargetId};

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedNeed(pub TargetId, pub bool, pub Need);
