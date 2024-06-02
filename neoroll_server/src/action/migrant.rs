use neoroll_world::{entity::structure::Structure, gameplay::tribe::TribeId};

use crate::{
    shortcut,
    state::{State, StateChange},
};

use super::{ActionId, BodyTick, NextTick};

const TICK_FREQUENCY: u64 = 2;

#[derive(Debug, PartialEq)]
pub struct IncomingMigrant;

impl IncomingMigrant {
    fn tick_tribe(&self, tribe_id: &TribeId, state: &State) -> Vec<StateChange> {
        let max_population = 5; // Later, it will be computed by complex algorithm
        let population_count = state
            .world()
            .tribe_creature_ids(tribe_id)
            .unwrap_or(&vec![])
            .len();

        if population_count < max_population {
            if let Some(campfire) = state
                .game()
                .tribe_structures(tribe_id, Some(Structure::Campfire))
                .first()
            {
                return shortcut::creature::new_creature(*tribe_id, *campfire.point());
            }
        }

        vec![]
    }
}

impl BodyTick<()> for IncomingMigrant {
    fn tick(&self, _id: ActionId, state: &State) -> (super::NextTick, Vec<StateChange>) {
        let mut changes = vec![];

        for tribe_id in state.game().tribe_ids() {
            changes.extend(self.tick_tribe(&tribe_id, state))
        }

        (NextTick(*state.frame_i() + TICK_FREQUENCY), changes)
    }

    fn apply(&mut self, _change: ()) {}
}
