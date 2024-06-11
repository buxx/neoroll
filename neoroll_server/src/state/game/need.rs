use neoroll_world::gameplay::need::Need;

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedNeed(pub bool, pub Need);
