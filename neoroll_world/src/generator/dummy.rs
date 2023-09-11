use rand::seq::SliceRandom;

use crate::{space::RegionCoordinate, state::World, tile::RegionTile};

use super::WorldGenerator;

#[derive(Default)]
pub struct DummyWorldGenerator {
    forced_grass_lands: Vec<RegionCoordinate>,
}

impl DummyWorldGenerator {
    pub fn forced_grass_lands(mut self, value: Vec<RegionCoordinate>) -> Self {
        self.forced_grass_lands = value;
        self
    }
}

impl WorldGenerator for DummyWorldGenerator {
    // Draw a grassland squad in the middle

    fn region(&self, _world: &World, at: RegionCoordinate) -> RegionTile {
        if self.forced_grass_lands.contains(&at) {
            return RegionTile::GrassLand;
        }

        *[RegionTile::Forest, RegionTile::GrassLand]
            .choose(&mut rand::thread_rng())
            .unwrap_or(&RegionTile::GrassLand)
    }
}
