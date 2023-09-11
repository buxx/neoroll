use rand::seq::SliceRandom;

use crate::{space::RegionCoordinate, state::World, tile::RegionTile};

use super::WorldGenerator;

pub struct DummyWorldGenerator;

impl WorldGenerator for DummyWorldGenerator {
    fn region(&self, _world: &World, _at: RegionCoordinate) -> RegionTile {
        *[RegionTile::Forest, RegionTile::GrassLand]
            .choose(&mut rand::thread_rng())
            .unwrap_or(&RegionTile::GrassLand)
    }
}
