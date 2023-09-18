use crate::space::{world::EntireWorld, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI};
use rand::Rng;

use super::{
    element::Element,
    sector::{Sector, SectorRelativePoint},
    Map, MAP_TILE_FACTOR,
};

pub struct WorldMapBuilder<'a> {
    world: &'a EntireWorld,
}

impl<'a> WorldMapBuilder<'a> {
    pub fn new(world: &'a EntireWorld) -> Self {
        Self { world }
    }

    pub fn build(&self) -> Map {
        let mut sectors = vec![];
        let lines = self.world.lines() / MAP_TILE_FACTOR;
        let columns = self.world.columns() / MAP_TILE_FACTOR;

        for line in 0..lines {
            for column in 0..columns {
                sectors.push(self.sector(line, column));
            }
        }

        Map::new(sectors, lines, columns)
    }

    fn sector(&self, start_world_row: usize, start_world_col: usize) -> Sector {
        let mut structures = vec![];
        let mut rng = rand::thread_rng();

        for row_i in start_world_row..start_world_row + MAP_TILE_FACTOR {
            for col_i in start_world_col..start_world_col + MAP_TILE_FACTOR {
                if let Some(structure) = self.world.structure(&AbsoluteWorldPoint(
                    AbsoluteWorldRowI(row_i as isize),
                    AbsoluteWorldColI(col_i as isize),
                )) {
                    structures.push(structure);
                }
            }
        }

        // This zone of code is very simple for now
        let mut items = vec![];
        let ratio = structures.len() as f32 / (MAP_TILE_FACTOR * MAP_TILE_FACTOR) as f32;
        for _ in 0..(ratio * 15.) as usize {
            items.push((
                SectorRelativePoint(rng.gen::<f32>(), rng.gen::<f32>()),
                Element::Tree,
            ));
        }

        Sector::new(items)
    }
}
