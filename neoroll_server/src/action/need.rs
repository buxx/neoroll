use neoroll_world::{
    entity::structure::Structure,
    gameplay::{
        material::Material,
        need::Need,
        target::{
            need::{ComputedNeed, NeedState, WaitingReason},
            Target, TargetId,
        },
        tribe::TribeId,
        Quantity,
    },
};

use crate::{
    run::TICK_BASE_PERIOD,
    state::{game::GameChange, State, StateChange},
    target::IntoQuantity,
};

use super::{ActionId, BodyTick, NextTick};

const TICK_PERIOD: u64 = TICK_BASE_PERIOD * 10;

#[derive(Debug, PartialEq)]
pub struct ComputeTribeNeeds {
    tribe_id: TribeId,
}
impl ComputeTribeNeeds {
    pub(crate) fn new(tribe_id: TribeId) -> Self {
        Self { tribe_id }
    }
}

impl BodyTick<ComputeTribeNeedsChange> for ComputeTribeNeeds {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let game: std::sync::RwLockReadGuard<crate::state::game::GameState> = state.game();
        let tribe_settings = game.tribe_settings().get(&self.tribe_id).unwrap();

        let all_needs = tribe_settings
            .targets()
            .iter()
            .map(|(id, t)| t.needs(id, &self.tribe_id, state))
            .collect::<Vec<Vec<(TargetId, Need)>>>()
            .concat();
        let computed_needs = all_needs
            .iter()
            .map(|(target_id, need)| {
                ComputedNeed(
                    *target_id,
                    need.satisfied(&self.tribe_id, state),
                    need.clone(),
                )
            })
            .collect::<Vec<ComputedNeed>>();

        (
            NextTick(*state.frame_i() + TICK_PERIOD),
            vec![StateChange::Game(GameChange::SetTribeNeeds(
                self.tribe_id,
                computed_needs,
            ))],
        )
    }

    fn apply(&mut self, _change: ComputeTribeNeedsChange) {}
}

#[derive(Debug)]
pub enum ComputeTribeNeedsChange {}

trait IntoNeeds {
    fn needs(
        &self,
        target_id: &TargetId,
        tribe_id: &TribeId,
        state: &State,
    ) -> Vec<(TargetId, Need)>;
}

impl IntoNeeds for Target {
    fn needs(
        &self,
        target_id: &TargetId,
        tribe_id: &TribeId,
        state: &State,
    ) -> Vec<(TargetId, Need)> {
        match self {
            Target::KeepStock(material, quantity) => {
                vec![(
                    *target_id,
                    Need::MaterialInStorages(*material, quantity.resolve_quantity(state, tribe_id)),
                )]
            }
        }
    }
}

trait Satisfied {
    fn satisfied(&self, tribe_id: &TribeId, state: &State) -> NeedState;
}

impl Satisfied for Need {
    fn satisfied(&self, tribe_id: &TribeId, state: &State) -> NeedState {
        let game = state.game();
        let world = state.world();

        match self {
            Need::MaterialInStorages(material, quantity) => {
                let covered = game
                    .tribe_structures(tribe_id, Some(Structure::Storage))
                    .iter()
                    .map(|s| world.materials_on(s.point(), Some(*material)))
                    .collect::<Vec<Vec<&(Material, Quantity)>>>()
                    .concat()
                    .iter()
                    .map(|(_, q)| q.clone())
                    .sum::<Quantity>()
                    .0
                    >= quantity.0;

                if covered {
                    return NeedState::Covered;
                }

                NeedState::Waiting(WaitingReason::NotEnoughMaterial(material.clone()))
            }
        }
    }
}
