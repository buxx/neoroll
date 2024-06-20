use neoroll_world::gameplay::{target::TargetQuantity, tribe::TribeId, Quantity};

use crate::state::State;

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
