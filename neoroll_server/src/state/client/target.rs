use std::collections::HashMap;

use neoroll_world::{
    entity::{creature::Creature, structure::Structure},
    gameplay::{
        job::Job,
        material::Material,
        need::Need,
        target::{ComputedTarget, Target, TargetId},
        tribe::TribeId,
        Quantity,
    },
};

use crate::{state::State, target::IntoQuantity};

#[derive(Debug, Clone, PartialEq)]
pub struct TargetsGameState {
    targets: HashMap<TargetId, ComputedTarget>,
}

impl TargetsGameState {
    pub fn new(targets: Vec<ComputedTarget>) -> Self {
        Self {
            targets: targets.iter().map(|t| (*t.id(), t.clone())).collect(),
        }
    }

    pub fn targets(&self) -> &HashMap<TargetId, ComputedTarget> {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut HashMap<TargetId, ComputedTarget> {
        &mut self.targets
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
        TargetsGameState::new(computed_targets)
    }
}
