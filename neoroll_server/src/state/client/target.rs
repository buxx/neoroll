use std::collections::HashMap;

use neoroll_world::gameplay::{
    target::{Target, TargetId},
    tribe::TribeId,
};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct TargetsGameState {
    targets: HashMap<TargetId, Target>,
}

impl TargetsGameState {
    pub fn new(targets: HashMap<TargetId, Target>) -> Self {
        Self { targets }
    }

    pub fn targets(&self) -> &HashMap<TargetId, Target> {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut HashMap<TargetId, Target> {
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
        let game = self.state.game();
        let tribe = game.tribe_settings().get(tribe_id).unwrap();
        let targets = tribe.targets().clone();

        TargetsGameState::new(targets)
    }
}
