use crate::{gateway::ClientId, state::State};

use super::{
    build::BuildGameStateBuilder, human::HumanGameStateBuilder, material::MaterialsStateBuilder,
    target::TargetGameStateBuilder, ClientGameState,
};

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
        let target = TargetGameStateBuilder::new(self.state).build(tribe_id);
        let needs = game.tribe_needs().get(tribe_id).unwrap_or(&vec![]).clone();
        let materials = MaterialsStateBuilder::new(self.state).build(tribe_id);

        ClientGameState::new(*tribe_id, human, build, target, needs, materials)
    }
}
