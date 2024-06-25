use neoroll_world::{
    entity::{creature::Creature, structure::Structure},
    gameplay::{
        job::Job,
        material::Material,
        need::Need,
        target::{ComputedTarget, Target},
        tribe::TribeId,
        Quantity,
    },
};

use crate::{
    run::TICK_BASE_PERIOD,
    state::{
        game::{ComputedTargetChange, GameChange},
        State, StateChange,
    },
    target::IntoQuantity,
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
        let game = state.game();
        let world = state.world();
        let tribe = game.tribe_settings().get(&self.tribe_id).unwrap();
        let targets = tribe.targets().clone();
        let mut computed_targets = vec![];

        for (target_id, target) in targets {
            let needs = target.satisfaction_needs(&self.tribe_id, &state);
            let satisfied = needs.iter().all(|n| n.satisfied(&self.tribe_id, &state));

            let affected = world
                .tribe_creature_ids(&self.tribe_id)
                .unwrap_or(&vec![])
                .iter()
                .map(|i| world.creatures().get(i).expect("Id just retrieved"))
                .filter(|c| c.job() == &Job::from(&target))
                .collect::<Vec<&Creature>>()
                .len();

            computed_targets.push(ComputedTarget::new(
                target_id,
                target,
                satisfied,
                affected,
                needs.clone(),
            ));
        }

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

// TODO Move these trait elsewhere ?
trait IntoSatisfactionNeeds {
    fn satisfaction_needs(&self, tribe_id: &TribeId, state: &State) -> Vec<Need>;
}

impl IntoSatisfactionNeeds for Target {
    fn satisfaction_needs(&self, tribe_id: &TribeId, state: &State) -> Vec<Need> {
        match self {
            Target::KeepStock(material, quantity) => {
                vec![Need::MaterialInStorages(
                    *material,
                    quantity.resolve_quantity(state, tribe_id),
                )]
            }
        }
    }
}

trait Satisfied {
    fn satisfied(&self, tribe_id: &TribeId, state: &State) -> bool;
}

impl Satisfied for Need {
    fn satisfied(&self, tribe_id: &TribeId, state: &State) -> bool {
        let game = state.game();
        let world = state.world();

        match self {
            Need::MaterialInStorages(material, quantity) => {
                game.tribe_structures(tribe_id, Some(Structure::Storage))
                    .iter()
                    .map(|s| world.materials_on(s.point(), Some(*material)))
                    .collect::<Vec<Vec<&(Material, Quantity)>>>()
                    .concat()
                    .iter()
                    .map(|(_, q)| q.clone())
                    .sum::<Quantity>()
                    .0
                    >= quantity.0
            }
        }
    }
}
