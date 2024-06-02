pub mod builder;
use std::fmt::Display;

use neoroll_world::gameplay::tribe::TribeId;

#[derive(Debug, Clone, PartialEq)]
pub struct ClientGameState {
    tribe_id: TribeId,
    human: HumanGameState,
    build: BuildGameState,
}

impl ClientGameState {
    pub fn new(tribe_id: TribeId, human: HumanGameState, build: BuildGameState) -> Self {
        Self {
            tribe_id,
            human,
            build,
        }
    }

    pub fn tribe_id(&self) -> &TribeId {
        &self.tribe_id
    }

    pub fn human(&self) -> &HumanGameState {
        &self.human
    }

    pub fn build(&self) -> &BuildGameState {
        &self.build
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HumanGameState {
    human_count: HumanCount,
}

impl HumanGameState {
    fn new(human_count: HumanCount) -> Self {
        Self { human_count }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuildGameState {
    can_build_campfire: bool,
}

impl BuildGameState {
    fn new(can_build_campfire: bool) -> Self {
        Self { can_build_campfire }
    }

    pub fn can_build_campfire(&self) -> bool {
        self.can_build_campfire
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HumanCount(u16);

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
