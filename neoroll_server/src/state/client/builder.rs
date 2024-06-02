use neoroll_world::{entity::structure::Structure, gameplay::tribe::TribeId};

use crate::{gateway::ClientId, state::State};

use super::{BuildGameState, ClientGameState, HumanCount, HumanGameState};

pub struct ClientGameStateBuilder<'a> {
    state: &'a State,
}

impl<'a> ClientGameStateBuilder<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn build(self, client_id: &ClientId) -> ClientGameState {
        let game = self.state.game();
        let tribe_id = game.client_tribe_id(client_id).unwrap();
        let human = HumanGameStateBuilder::new(self.state).build(tribe_id);
        let build = BuildGameStateBuilder::new(self.state).build(tribe_id);

        ClientGameState::new(*tribe_id, human, build)
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

        BuildGameState::new(can_build_campfire)
    }
}
