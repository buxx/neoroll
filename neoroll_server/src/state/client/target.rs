use neoroll_world::gameplay::{target::Target, tribe::TribeId};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct TargetsGameState {
    targets: Vec<Target>,
}

impl TargetsGameState {
    pub fn new(targets: Vec<Target>) -> Self {
        Self { targets }
    }

    pub fn targets(&self) -> &[Target] {
        &self.targets
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
        let targets = tribe.targets().to_vec();

        TargetsGameState::new(targets)
    }
}
