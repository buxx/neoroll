use neoroll_world::{
    entity::{creature::Creature, structure::Structure},
    gameplay::{
        job::Job,
        material::Material,
        need::Need,
        target::{ComputedTarget, Target, TargetQuantity},
        tribe::TribeId,
        Quantity,
    },
};

use crate::state::State;

pub struct ComputedTargetBuilder<'a> {
    state: &'a State,
    tribe_id: TribeId,
}

impl<'a> ComputedTargetBuilder<'a> {
    pub fn new(state: &'a State, tribe_id: TribeId) -> Self {
        Self { state, tribe_id }
    }

    pub fn build(&self) -> Vec<ComputedTarget> {
        let game = self.state.game();
        let world = self.state.world();
        let tribe = game.tribe_settings().get(&self.tribe_id).unwrap();
        let targets = tribe.targets().clone();
        let mut computed_targets = vec![];

        for (target_id, target) in targets {
            let needs = target
                .target()
                .satisfaction_needs(&self.tribe_id, self.state);
            let satisfied = needs
                .iter()
                .all(|n| n.satisfied(&self.tribe_id, self.state));

            let affected = world
                .tribe_creature_ids(&self.tribe_id)
                .unwrap_or(&vec![])
                .iter()
                .map(|i| world.creatures().get(i).expect("Id just retrieved"))
                .filter(|c| c.job() == &Job::from(target.target()))
                .collect::<Vec<&Creature>>()
                .len();

            computed_targets.push(ComputedTarget::new(
                target_id,
                target.target().clone(),
                satisfied,
                affected,
                needs.clone(),
                target.priority(),
            ));
        }

        computed_targets
    }
}

pub trait IntoQuantity {
    fn resolve_quantity(&self, state: &State, tribe_id: &TribeId) -> Quantity;
}

impl IntoQuantity for TargetQuantity {
    fn resolve_quantity(&self, state: &State, tribe_id: &TribeId) -> Quantity {
        match self {
            TargetQuantity::Fixed(quantity) => quantity.clone(),
            TargetQuantity::PerHuman(quantity) => {
                // FIXME: Filter Humans ?
                let tribe_humans = state
                    .world()
                    .tribe_creature_ids(tribe_id)
                    .unwrap_or(&vec![])
                    .len();
                Quantity(quantity.0 * tribe_humans as u64)
            }
        }
    }
}

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
