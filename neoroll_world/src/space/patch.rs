use std::collections::HashMap;

use crate::{
    entity::{creature::PartialCreature, floor::Floor, ground::Ground, structure::Structure},
    gameplay::{material::Material, Quantity},
};

use super::{area::WorldArea, world::World, AbsoluteWorldPoint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewLayers {
    grounds: HashMap<AbsoluteWorldPoint, Ground>,
    floors: HashMap<AbsoluteWorldPoint, Floor>,
    structures: HashMap<AbsoluteWorldPoint, Structure>,
    creatures: Vec<PartialCreature>,
    materials: HashMap<AbsoluteWorldPoint, Vec<(Material, Quantity)>>,
}

impl NewLayers {
    pub fn from_world_area(world: &World, area: &WorldArea, ignore: &WorldArea) -> Self {
        let mut grounds = HashMap::new();
        let mut floors = HashMap::new();
        let mut structures = HashMap::new();
        let mut materials = HashMap::new();

        for point in area.points() {
            if !ignore.contains(&point) {
                if let Some(ground) = world.ground(&point) {
                    grounds.insert(point, ground.clone());
                }
                if let Some(floor) = world.floor(&point) {
                    floors.insert(point, floor.clone());
                }
                if let Some(structure) = world.structure(&point) {
                    structures.insert(point, structure.clone());
                }
                if let Some(material) = world.material(&point) {
                    materials.insert(point, material.clone());
                }
            }
        }

        Self {
            grounds,
            floors,
            structures,
            // FIXME BS NOW: creatures must be filled by taking creature only inside given area
            creatures: world
                .creatures()
                .values()
                .cloned()
                .map(|c| c.into())
                .collect(),
            materials,
        }
    }

    pub fn ground(&self, point: &AbsoluteWorldPoint) -> Option<&Ground> {
        self.grounds.get(point)
    }

    pub fn floor(&self, point: &AbsoluteWorldPoint) -> Option<&Floor> {
        self.floors.get(point)
    }

    pub fn material(&self, point: &AbsoluteWorldPoint) -> Option<&Vec<(Material, Quantity)>> {
        self.materials.get(point)
    }

    pub fn structure(&self, point: &AbsoluteWorldPoint) -> Option<&Structure> {
        self.structures.get(point)
    }

    pub fn len(&self) -> usize {
        self.grounds.len() + self.floors.len() + self.structures.len()
    }

    pub fn creatures(&self) -> &[PartialCreature] {
        &self.creatures
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
