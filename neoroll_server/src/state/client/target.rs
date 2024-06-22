use std::collections::HashMap;

use neoroll_world::{
    entity::creature::Creature,
    gameplay::{
        job::Job,
        target::{
            need::{ComputedNeed, NeedState},
            ComputedTarget, TargetId, TargetState,
        },
        tribe::TribeId,
    },
};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct TargetsGameState {
    targets: HashMap<TargetId, ComputedTarget>,
    needs: Vec<ComputedNeed>,
}

impl TargetsGameState {
    pub fn new(targets: Vec<ComputedTarget>, needs: Vec<ComputedNeed>) -> Self {
        Self {
            targets: targets.iter().map(|t| (*t.id(), t.clone())).collect(),
            needs,
        }
    }

    pub fn targets(&self) -> &HashMap<TargetId, ComputedTarget> {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut HashMap<TargetId, ComputedTarget> {
        &mut self.targets
    }

    pub fn needs(&self) -> &[ComputedNeed] {
        &self.needs
    }
}

pub struct TargetGameStateBuilder<'a> {
    state: &'a State,
}

impl<'a> TargetGameStateBuilder<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn build(self, tribe_id: &TribeId) -> TargetsGameState {
        let game = self.state.game();
        let world = self.state.world();
        let tribe = game.tribe_settings().get(tribe_id).unwrap();
        let targets = tribe.targets().clone();
        let needs = game.tribe_needs().get(tribe_id).unwrap_or(&vec![]).clone();
        let mut computed_targets = vec![];

        for (target_id, target) in targets {
            let target_needs: Vec<&ComputedNeed> =
                needs.iter().filter(|n| n.0 == target_id).collect();
            let all_need_satisfied = target_needs.iter().all(|n| n.1.is_satisfied());
            let mut target_affected = 0;
            let mut waiting_reasons = vec![];

            for need in target_needs {
                let affected = world
                    .tribe_creature_ids(tribe_id)
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|i| world.creatures().get(i).expect("Id just retrieved"))
                    .filter(|c| c.job() == &Job::from(&need.2))
                    .collect::<Vec<&Creature>>()
                    .len();
                target_affected += affected;

                if let NeedState::Waiting(reason) = &need.1 {
                    waiting_reasons.push(reason.clone());
                }
            }

            let state = if all_need_satisfied {
                TargetState::Covered
            } else if target_affected > 0 {
                TargetState::InProgress(waiting_reasons)
            } else {
                TargetState::Waiting(waiting_reasons)
            };

            computed_targets.push(ComputedTarget::new(
                target_id,
                target,
                state,
                target_affected,
            ))
        }

        TargetsGameState::new(computed_targets, needs)
    }
}
