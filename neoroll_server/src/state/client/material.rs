use std::collections::HashMap;

use neoroll_world::{
    entity::structure::Structure,
    gameplay::{material::Material, tribe::TribeId, Quantity},
    space::AbsoluteWorldPoint,
};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialsState {
    points: Vec<(AbsoluteWorldPoint, Vec<(Material, Quantity)>)>,
}
impl MaterialsState {
    fn new(points: Vec<(AbsoluteWorldPoint, Vec<(Material, Quantity)>)>) -> Self {
        Self { points }
    }

    pub fn storages(&self) -> &[(AbsoluteWorldPoint, Vec<(Material, Quantity)>)] {
        &self.points
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
        let mut points = vec![];
        for storage in self
            .state
            .game()
            .tribe_structures(tribe_id, Some(Structure::Storage))
        {
            let content = self
                .state
                .world()
                .materials_on(storage.point(), None)
                .iter()
                .map(|(m, q)| (*m, q.clone()))
                .collect();
            points.push((*storage.point(), content));
        }

        MaterialsState::new(points)
    }
}
