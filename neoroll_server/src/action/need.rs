use neoroll_world::{
    entity::structure::Structure,
    gameplay::{
        material::{Material, Resource},
        need::Need,
        target::Target,
        tribe::TribeId,
        Quantity,
    },
};

use crate::{
    run::TICK_BASE_PERIOD,
    state::{
        game::{need::ComputedNeed, GameChange},
        State, StateChange,
    },
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
            .map(|t| t.needs(&self.tribe_id, state))
            .collect::<Vec<Vec<Need>>>()
            .concat();
        let computed_needs = all_needs
            .iter()
            .map(|need| ComputedNeed(need.satisfied(&self.tribe_id, state), need.clone()))
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
    fn needs(&self, tribe_id: &TribeId, state: &State) -> Vec<Need>;
}

impl IntoNeeds for Target {
    fn needs(&self, tribe_id: &TribeId, state: &State) -> Vec<Need> {
        let world = state.world();

        match self {
            Target::KeepStock(material, quantity) => match material {
                Material::Resource(Resource::Food) => {
                    // TODO: Filter Humans ?
                    let tribe_humans = world.tribe_creature_ids(tribe_id).unwrap_or(&vec![]).len();
                    vec![Need::MaterialInStorages(
                        *material,
                        Quantity(quantity.0 .0 * tribe_humans as u64),
                    )]
                }
            },
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
                    .map(|(_, q)| *q)
                    .sum::<Quantity>()
                    .0
                    >= quantity.0
            }
        }
    }
}
