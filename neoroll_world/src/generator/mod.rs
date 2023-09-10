use crate::{space::RegionCoordinate, state::World, tile::RegionTile};

pub mod dummy;

pub trait WorldGenerator {
    fn region(&self, world: &World, at: RegionCoordinate) -> RegionTile;
}
