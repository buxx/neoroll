use std::collections::HashMap;

use crate::entity::{creature::Creature, floor::Floor, ground::Ground, structure::Structure};

use super::{area::WorldArea, world::World, AbsoluteWorldPoint};

pub struct NewLayers {
    grounds: HashMap<AbsoluteWorldPoint, Ground>,
    floors: HashMap<AbsoluteWorldPoint, Floor>,
    structures: HashMap<AbsoluteWorldPoint, Structure>,
    creatures: Vec<Creature>,
}

impl NewLayers {
    pub fn from_world_area(world: &World, area: &WorldArea, ignore: &WorldArea) -> Self {
        let mut grounds = HashMap::new();
        let mut floors = HashMap::new();
        let mut structures = HashMap::new();

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
            }
        }

        Self {
            grounds,
            floors,
            structures,
            // FIXME BS NOW: creatures must be filled by taking creature only inside given area
            creatures: world.creatures().values().cloned().collect(),
        }
    }

    pub fn ground(&self, point: &AbsoluteWorldPoint) -> Option<&Ground> {
        self.grounds.get(point)
    }

    pub fn floor(&self, point: &AbsoluteWorldPoint) -> Option<&Floor> {
        self.floors.get(point)
    }

    pub fn structure(&self, point: &AbsoluteWorldPoint) -> Option<&Structure> {
        self.structures.get(point)
    }

    pub fn len(&self) -> usize {
        self.grounds.len() + self.floors.len() + self.structures.len()
    }

    pub fn creatures(&self) -> &[Creature] {
        &self.creatures
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
