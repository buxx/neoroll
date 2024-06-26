use neoroll_world::gameplay::tribe::TribeId;

use crate::{
    run::TICK_BASE_PERIOD,
    state::{
        game::{ComputedTargetChange, GameChange},
        State, StateChange,
    },
    target::ComputedTargetBuilder,
};

use super::{ActionId, BodyTick, NextTick};

const TICK_PERIOD: u64 = TICK_BASE_PERIOD * 2;

#[derive(Debug, PartialEq)]
pub struct ComputeTargets {
    tribe_id: TribeId,
}

impl ComputeTargets {
    pub fn new(tribe_id: TribeId) -> Self {
        Self { tribe_id }
    }
}

impl BodyTick<ComputeTargetsChange> for ComputeTargets {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let computed_targets = ComputedTargetBuilder::new(&state, self.tribe_id).build();

        (
            NextTick(*state.frame_i() + TICK_PERIOD),
            vec![StateChange::Game(GameChange::ComputedTarget(
                self.tribe_id,
                ComputedTargetChange::Set(computed_targets),
            ))],
        )
    }

    fn apply(&mut self, _change: ComputeTargetsChange) {}
}

#[derive(Debug)]
pub enum ComputeTargetsChange {}
