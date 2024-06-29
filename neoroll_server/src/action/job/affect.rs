use neoroll_world::{
    entity::{creature::CreatureChange, structure::Structure},
    gameplay::{
        job::Job,
        target::{ComputedTarget, Target, WaitingReason},
        tribe::TribeId,
    },
    space::world::WorldChange,
};

use crate::{
    action::{Action, ActionId, BodyTick, NextTick},
    run::TICK_BASE_PERIOD,
    state::{
        game::{GameChange, WaitingChange},
        State, StateChange,
    },
    utils::CreaturesJobUtils,
};

const TICK_FREQUENCY: u64 = TICK_BASE_PERIOD * 5;

#[derive(Debug, PartialEq)]
pub struct AffectJob {
    tribe_id: TribeId,
}
impl AffectJob {
    fn solve_target(&self, state: &State, target: &ComputedTarget) -> Vec<StateChange> {
        let mut changes = vec![];

        if target.covered() {
            return self.disband_target_worker(target, state);
        }

        if let Some(waitings) = self.global_waitings(state, target.target()) {
            return vec![StateChange::Game(GameChange::Waiting(
                self.tribe_id,
                WaitingChange::Set(*target.id(), waitings),
            ))];
        }

        let world = state.world();
        let creatures = world.tribe_creatures(&self.tribe_id);
        let idles = creatures.filter_job(&Job::Idle);

        if idles.is_empty() {
            return vec![StateChange::Game(GameChange::Waiting(
                self.tribe_id,
                WaitingChange::Set(*target.id(), vec![WaitingReason::NotEnoughWorker]),
            ))];
        }

        // Try to affect
        let job = Job::from(target.target());
        for human in world.tribe_creatures(&self.tribe_id).filter_job(&Job::Idle) {
            changes.extend(vec![StateChange::World(WorldChange::Creature(
                *human.id(),
                CreatureChange::SetJob(job.clone()),
            ))])
        }

        changes
    }

    pub fn global_waitings(&self, state: &State, target: &Target) -> Option<Vec<WaitingReason>> {
        match target {
            Target::KeepStock(_, _) => {
                if state
                    .game()
                    .tribe_structures(&self.tribe_id, Some(Structure::Storage))
                    .first()
                    .is_none()
                {
                    return Some(vec![WaitingReason::NeedOwnedStructure(Structure::Storage)]);
                }
            }
        }

        None
    }

    fn disband_target_worker(&self, target: &ComputedTarget, state: &State) -> Vec<StateChange> {
        let world = state.world();
        let target_job = Job::from(target.target());
        let mut changes = vec![];

        for human in world
            .tribe_creatures(&self.tribe_id)
            .filter_job(&target_job)
        {
            changes.push(StateChange::World(WorldChange::Creature(
                *human.id(),
                CreatureChange::SetJob(Job::Idle),
            )));
        }

        changes
    }
}

impl BodyTick<AffectJobChange> for AffectJob {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];
        let game = state.game();

        for target in game.tribe_targets().get(&self.tribe_id).unwrap_or(&vec![]) {
            changes.extend(self.solve_target(state, target))
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
