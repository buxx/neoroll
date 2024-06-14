use std::collections::HashMap;

use neoroll_world::{
    entity::structure::Structure,
    gameplay::{material::Material, tribe::TribeId, Quantity},
};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialsState {
    total: Vec<(Material, Quantity)>,
}
impl MaterialsState {
    fn new(total: Vec<(Material, Quantity)>) -> Self {
        Self { total }
    }

    pub fn total(&self) -> &[(Material, Quantity)] {
        &self.total
    }
}

pub struct MaterialsStateBuilder<'a> {
    state: &'a State,
}

impl<'a> MaterialsStateBuilder<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn build(self, tribe_id: &TribeId) -> MaterialsState {
        // FIXME BS NOW: count only tribe human creatures !
        let mut total: HashMap<Material, Quantity> = HashMap::new();
        for storage in self
            .state
            .game()
            .tribe_structures(tribe_id, Some(Structure::Storage))
        {
            for (material, quantity) in self.state.world().materials_on(storage.point(), None) {
                total.entry(*material).or_default().0 += quantity.0;
            }
        }

        MaterialsState::new(
            total
                .iter()
                .map(|(k, v)| (*k, v.clone()))
                .collect::<Vec<(Material, Quantity)>>(),
        )
    }
}
