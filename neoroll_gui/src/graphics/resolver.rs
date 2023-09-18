use neoroll_world::entity::{floor::Floor, ground::Ground, structure::Structure};

use super::{
    tileset::world::{floor_tile_name, ground_tile_name, structure_tile_name},
    TileName,
};

pub struct LayersResolver;

impl LayersResolver {
    pub fn resolve(
        &self,
        ground: &Option<Ground>,
        floor: &Option<Floor>,
        structure: &Option<Structure>,
    ) -> Vec<TileName> {
        let mut tiles = vec![];

        if !floor.as_ref().unwrap_or(&Floor::Nothing).hide() {
            if let Some(ground) = ground {
                tiles.push(ground_tile_name(ground))
            }
        }

        if !structure.as_ref().unwrap_or(&Structure::Nothing).hide() {
            if let Some(floor) = floor {
                tiles.push(floor_tile_name(floor))
            }
        }

        if let Some(structure) = structure {
            tiles.push(structure_tile_name(structure))
        }

        tiles
    }
}
