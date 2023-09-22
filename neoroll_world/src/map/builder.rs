use super::{
    element::Element,
    sector::{Sector, SectorRelativePoint},
    Map, MAP_TILE_FACTOR,
};
use crate::space::{world::EntireWorld, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI};
use rand::seq::SliceRandom;
use rand::Rng;

pub struct MapBuilder<'a> {
    world: &'a EntireWorld,
}

impl<'a> MapBuilder<'a> {
    pub fn new(world: &'a EntireWorld) -> Self {
        Self { world }
    }

    pub fn build(&self) -> Map {
        let mut sectors = vec![];
        let world_lines = self.world.lines();
        let world_columns = self.world.columns();

        for line in (0..world_lines).step_by(MAP_TILE_FACTOR) {
            for column in (0..world_columns).step_by(MAP_TILE_FACTOR) {
                sectors.push(self.sector(line, column));
            }
        }

        // If number of lines/columns is not divisible by MAP_TILE_FACTOR
        // a "not finished" sector will represent last part. So,
        // lines/columns count must count it
        let lines = (world_lines as f32 / MAP_TILE_FACTOR as f32).ceil() as usize;
        let columns = (world_columns as f32 / MAP_TILE_FACTOR as f32).ceil() as usize;

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

        let new_items = if ratio > 0.60 {
            vec![(
                SectorRelativePoint(rng.gen_range(0.35..0.55), rng.gen_range(0.35..0.55)),
                [
                    Element::Tree3a,
                    Element::Tree3b,
                    Element::Tree3c,
                    Element::Tree4a,
                    Element::Tree4b,
                    Element::Tree4c,
                    Element::Tree4d,
                ]
                .choose(&mut rng)
                .unwrap_or(&Element::Tree4a)
                .clone(),
            )]
        } else if ratio > 0.50 {
            vec![(
                SectorRelativePoint(rng.gen_range(0.25..0.65), rng.gen_range(0.25..0.65)),
                [Element::Tree2a, Element::Tree2b]
                    .choose(&mut rng)
                    .unwrap_or(&Element::Tree2a)
                    .clone(),
            )]
        } else if ratio > 0.05 {
            vec![(
                SectorRelativePoint(rng.gen_range(0.25..0.65), rng.gen_range(0.25..0.65)),
                [Element::Tree1a, Element::Tree1b, Element::Tree1c]
                    .choose(&mut rng)
                    .unwrap_or(&Element::Tree1a)
                    .clone(),
            )]
        } else {
            vec![]
        };
        items.extend(new_items);

        Sector::new(items)
    }
}
