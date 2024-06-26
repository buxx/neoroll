use std::collections::HashMap;

use neoroll_world::gameplay::{
    target::{ComputedTarget, TargetId, WaitingReason},
    tribe::TribeId,
};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct TargetsGameState {
    targets: HashMap<TargetId, ComputedTarget>,
    waitings: HashMap<TargetId, Vec<WaitingReason>>,
}

impl TargetsGameState {
    pub fn new(
        targets: Vec<ComputedTarget>,
        waitings: HashMap<TargetId, Vec<WaitingReason>>,
    ) -> Self {
        Self {
            targets: targets.iter().map(|t| (*t.id(), t.clone())).collect(),
            waitings,
        }
    }

    pub fn targets(&self) -> &HashMap<TargetId, ComputedTarget> {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut HashMap<TargetId, ComputedTarget> {
        &mut self.targets
    }

    pub fn waitings(&self) -> &HashMap<TargetId, Vec<WaitingReason>> {
        &self.waitings
    }
}

pub struct TargetGameStateBuilder<'a> {
    state: &'a State,
}

impl<'a> TargetGameStateBuilder<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn build(self, tribe_id: &TribeId) -> TargetsGameState {
        let computed_targets = self
            .state
            .game()
            .tribe_targets()
            .get(tribe_id)
            .cloned()
            .unwrap_or_default();
        let waiting = self
            .state
            .game()
            .tribe_waitings()
            .get(tribe_id)
            .cloned()
            .unwrap_or_default();
        TargetsGameState::new(computed_targets, waiting)
    }
}
