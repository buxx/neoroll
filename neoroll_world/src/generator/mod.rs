use crate::{space::AbsoluteWorldPoint, state::EntireWorld, tile::RegionTile};

pub mod dummy;

pub trait WorldGenerator {
    fn region(&self, world: &EntireWorld, at: AbsoluteWorldPoint) -> RegionTile;
}
