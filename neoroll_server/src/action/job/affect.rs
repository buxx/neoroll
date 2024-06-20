use neoroll_world::{
    entity::creature::CreatureChange,
    gameplay::{job::Job, material::Material, need::Need, tribe::TribeId},
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
        let mut needs = game
            .tribe_needs()
            .get(&self.tribe_id)
            .unwrap_or(&default)
            .iter()
            .filter(|n| !n.0)
            .map(|n| &n.1)
            .collect::<Vec<&Need>>();
        let not_needs = game
            .tribe_needs()
            .get(&self.tribe_id)
            .unwrap_or(&default)
            .iter()
            .filter(|n| n.0)
            .map(|n| &n.1)
            .collect::<Vec<&Need>>();

        for human_id in world.tribe_creature_ids(&self.tribe_id).unwrap_or(&vec![]) {
            let human = world.creatures().get(human_id).unwrap();
            match human.job() {
                Job::Idle => {
                    if let Some(need) = needs.pop() {
                        match need {
                            Need::MaterialInStorages(material, _) => {
                                changes.push(StateChange::World(WorldChange::Creature(
                                    *human.id(),
                                    CreatureChange::SetJob(Job::from(material)),
                                )));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        for not_need in not_needs {
            match not_need {
                Need::MaterialInStorages(Material::Resource(resource), _) => {
                    // Disable job of workers
                    for human_id in world.tribe_creature_ids(&self.tribe_id).unwrap_or(&vec![]) {
                        match world.creatures().get(human_id).unwrap().job() {
                            Job::SearchResource(job_resource) => {
                                if resource == job_resource {
                                    changes.push(StateChange::World(WorldChange::Creature(
                                        *human_id,
                                        CreatureChange::SetJob(Job::Idle),
                                    )));
                                }
                            }
                            Job::Idle => {}
                        }
                    }
                }
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
