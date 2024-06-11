use neoroll_world::{entity::structure::Structure, gameplay::tribe::TribeId};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildGameState {
    can_build_campfire: bool,
    can_build_storage: bool,
}

impl BuildGameState {
    pub fn new(can_build_campfire: bool, can_build_storage: bool) -> Self {
        Self {
            can_build_campfire,
            can_build_storage,
        }
    }

    pub fn can_build_campfire(&self) -> bool {
        self.can_build_campfire
    }

    pub fn can_build_storage(&self) -> bool {
        self.can_build_storage
    }
}

pub struct BuildGameStateBuilder<'a> {
    state: &'a State,
}

impl<'a> BuildGameStateBuilder<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn build(self, tribe_id: &TribeId) -> BuildGameState {
        // In the future, we will manage migration, but for now, only one fire allowed
        let can_build_campfire = self
            .state
            .game()
            .tribe_structures(tribe_id, Some(Structure::Campfire))
            .is_empty();

        let can_build_storage = !self
            .state
            .game()
            .tribe_structures(tribe_id, Some(Structure::Campfire))
            .is_empty()
            && self
                .state
                .game()
                .tribe_structures(tribe_id, Some(Structure::Storage))
                .is_empty();

        BuildGameState::new(can_build_campfire, can_build_storage)
    }
}
