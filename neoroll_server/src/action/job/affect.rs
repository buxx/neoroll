use neoroll_world::{
    entity::{creature::CreatureChange, structure::Structure},
    gameplay::{
        job::Job,
        material::{Material, Resource},
        need::Need,
        target::Target,
        tribe::TribeId,
        Quantity,
    },
    space::world::WorldChange,
};

use crate::{
    action::{Action, ActionId, BodyTick, NextTick},
    run::TICK_BASE_PERIOD,
    state::{State, StateChange},
};

const TICK_FREQUENCY: u64 = TICK_BASE_PERIOD * 5;

#[derive(Debug, PartialEq)]
pub struct AffectJob {
    tribe_id: TribeId,
}

impl BodyTick<AffectJobChange> for AffectJob {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];
        let world = state.world();
        let game = state.game();

        let default = vec![];
        let needs = game
            .tribe_needs()
            .get(&self.tribe_id)
            .unwrap_or(&default)
            .iter()
            .filter(|n| !n.0)
            .map(|n| &n.1)
            .collect::<Vec<&Need>>();

        //
        //
        //

        // FIXME: compute regularly tribe state and use it to affect jobs
        // For now, affect simply the SearchFood job
        for human_id in world.tribe_creature_ids(&self.tribe_id).unwrap_or(&vec![]) {
            let human = world.creatures().get(human_id).unwrap();
            match human.job() {
                Job::SearchFood => {}
                Job::Idle => changes.push(StateChange::World(WorldChange::Creature(
                    *human.id(),
                    CreatureChange::SetJob(Job::SearchFood),
                ))),
            }
        }

        (NextTick(*state.frame_i() + TICK_FREQUENCY), changes)
    }

    fn apply(&mut self, _change: AffectJobChange) {}
}

#[derive(Debug)]
pub enum AffectJobChange {}

pub struct AffectJobBuilder {
    tribe_id: TribeId,
}

impl AffectJobBuilder {
    pub fn new(tribe_id: TribeId) -> Self {
        Self { tribe_id }
    }

    pub fn build(&self) -> Action {
        Action::AffectJob(AffectJob {
            tribe_id: self.tribe_id,
        })
    }
}
