use neoroll_world::{entity::creature::CreatureId, gameplay::job::Job};
use search::RealizeSearchResource;

use crate::{
    action::{Action, ActionId, BodyTick, NextTick},
    run::TICK_BASE_PERIOD,
    state::{State, StateChange},
};

pub mod search;

const TICK_FREQUENCY: u64 = TICK_BASE_PERIOD;

#[derive(Debug, PartialEq)]
pub struct RealizeJob {
    creature_id: CreatureId,
}

impl BodyTick<RealizeJobChange> for RealizeJob {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];
        let world = state.world();
        let creature = world.creatures().get(&self.creature_id).unwrap();

        match creature.job() {
            Job::Idle => {}
            Job::SearchResource(resource) => {
                changes.extend(RealizeSearchResource::new(creature, state, *resource).changes())
            }
        }

        (NextTick(*state.frame_i() + TICK_FREQUENCY), changes)
    }

    fn apply(&mut self, _change: RealizeJobChange) {}
}

#[derive(Debug)]
pub enum RealizeJobChange {}

pub struct RealizeJobBuilder {
    creature_id: CreatureId,
}

impl RealizeJobBuilder {
    pub fn new(creature_id: CreatureId) -> Self {
        Self { creature_id }
    }

    pub fn build(&self) -> Action {
        Action::RealizeJob(RealizeJob {
            creature_id: self.creature_id,
        })
    }
}
