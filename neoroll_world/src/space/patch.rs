use std::collections::HashMap;

use crate::entity::{floor::Floor, ground::Ground, structure::Structure};

use super::{area::WorldArea, world::EntireWorld, AbsoluteWorldPoint};

pub struct NewLayers {
    grounds: HashMap<AbsoluteWorldPoint, Ground>,
    floors: HashMap<AbsoluteWorldPoint, Floor>,
    structures: HashMap<AbsoluteWorldPoint, Structure>,
}

impl NewLayers {
    pub fn from_world_area(world: &EntireWorld, area: &WorldArea, ignore: &WorldArea) -> Self {
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

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
