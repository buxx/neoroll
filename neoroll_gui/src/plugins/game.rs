use bevy::prelude::*;
use neoroll_server::state::client::ClientGameState;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStateWrapper>();
    }
}

#[derive(Resource, Default)]
pub struct GameStateWrapper {
    state: Option<ClientGameState>,
}

impl GameStateWrapper {
    pub fn state(&self) -> &Option<ClientGameState> {
        &self.state
    }

    pub fn set_state(&mut self, state: Option<ClientGameState>) {
        println!("set_state");
        self.state = state;
    }
}
