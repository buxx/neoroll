use super::{
    element::Element,
    sector::{Sector, SectorRelativePoint},
    Map, MAP_TILE_FACTOR,
};
use crate::{
    entity::ground::Ground,
    space::{world::EntireWorld, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI},
    utils::{BlindFoldedMazesResolver, Direction},
};
use rand::seq::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;

pub struct MapBuilder<'a> {
    world: &'a EntireWorld,
    build_lakes: bool,
}

impl<'a> MapBuilder<'a> {
    pub fn new(world: &'a EntireWorld) -> Self {
        Self {
            world,
            build_lakes: false,
        }
    }

    pub fn build_lakes(mut self, value: bool) -> Self {
        self.build_lakes = value;
        self
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
        let lakes = if self.build_lakes {
            self.lakes()
        } else {
            vec![]
        };

        Map::new(sectors, lines, columns, lakes)
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

    pub fn lakes(&self) -> Vec<Vec<AbsoluteWorldPoint>> {
        let coasts = self.coasts();
        BlindFoldedMazesResolver::new(self.world, &coasts).resolve_all()
    }

    pub fn coasts(&self) -> Vec<AbsoluteWorldPoint> {
        let mut coasts = vec![];

        for row in 0..self.world.lines() {
            for col in 0..self.world.columns() {
                let point = AbsoluteWorldPoint(
                    AbsoluteWorldRowI(row as isize),
                    AbsoluteWorldColI(col as isize),
                );
                if self.world.ground(&point) == Some(&Ground::FreshWater) {
                    let all_neighbor_is_water = Direction::iter()
                        .map(|direction| self.world.ground(&point.next(&direction)))
                        .all(|g| g == Some(&Ground::FreshWater));

                    if !all_neighbor_is_water {
                        coasts.push(point);
                    }
                }
            }
        }

        coasts
    }
}

#[cfg(test)]
mod test {
    use crate::tests::str_map::WorldFromStrBuilder;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "00000
         01110
         01110
         01110
         00000",
         vec![(1, 1), (1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2), (3, 3)]
    )]
    #[case(
        "00000
         01110
         11111
         01110
         00000",
         vec![(1, 1), (1, 2), (1, 3), (2, 0), (2, 1), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3)]
    )]
    #[case(
        "110
         110
         000",
         vec![(0, 0), (0, 1), (1, 0), (1, 1)]
    )]
    fn test_map_coasts(#[case] map: &str, #[case] expected: Vec<(isize, isize)>) {
        // Given
        let world = WorldFromStrBuilder::new(map).build();

        // When
        let coasts = MapBuilder::new(&world).coasts();

        // Then
        let result = coasts
            .iter()
            .map(|p| (p.0 .0, p.1 .0))
            .collect::<Vec<(isize, isize)>>();
        debug_assert_eq!(result, expected)
    }
}
