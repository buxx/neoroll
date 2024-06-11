use std::fmt::Display;

use neoroll_world::gameplay::tribe::TribeId;

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct HumanGameState {
    human_count: HumanCount,
}

impl HumanGameState {
    pub fn new(human_count: HumanCount) -> Self {
        Self { human_count }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HumanCount(pub u16);

impl HumanCount {
    pub fn new(value: u16) -> Self {
        Self(value)
    }
}

impl Default for HumanCount {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Display for HumanCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

pub struct HumanGameStateBuilder<'a> {
    state: &'a State,
}

impl<'a> HumanGameStateBuilder<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn build(self, _tribe_id: &TribeId) -> HumanGameState {
        // FIXME BS NOW: count only tribe human creatures !
        let human_count = HumanCount(self.state.world().creatures().len() as u16);
        HumanGameState::new(human_count)
    }
}
