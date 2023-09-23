use super::{
    element::Element,
    sector::{Sector, SectorRelativePoint},
    Map, MAP_TILE_FACTOR,
};
use crate::{
    entity::ground::Ground,
    space::{world::EntireWorld, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI},
    utils::Direction,
};
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

    fn lakes(&self) -> Vec<Vec<AbsoluteWorldPoint>> {
        let mut lakes = vec![];

        // FIXME BS NOW : temp // Will must return the coats line
        let mut lake = vec![];

        for row in 0..self.world.lines() {
            for col in 0..self.world.columns() {
                let point = AbsoluteWorldPoint(
                    AbsoluteWorldRowI(row as isize),
                    AbsoluteWorldColI(col as isize),
                );
                if self.world.ground(&point) == Some(&Ground::FreshWater) {
                    // All 4 neighbor must be water too
                    if ![
                        self.world.ground(&point.next(&Direction::North)),
                        self.world.ground(&point.next(&Direction::Est)),
                        self.world.ground(&point.next(&Direction::West)),
                        self.world.ground(&point.next(&Direction::South)),
                    ]
                    .iter()
                    .all(|g| g == &Some(&Ground::FreshWater))
                    {
                        lake.push(point);
                    }
                }
            }
        }

        // FIXME BS NOW : temp
        lakes.push(lake);

        lakes
    }
}

#[cfg(test)]
mod test {
    use crate::{
        entity::{floor::Floor, ground::Ground},
        space::layer::{CompositeLayer, FilledLayer, Layers},
    };

    use super::*;
    use rstest::*;

    pub struct WorldFromStrBuilder<'a> {
        raw: &'a str,
    }

    impl<'a> WorldFromStrBuilder<'a> {
        pub fn new(raw: &'a str) -> Self {
            Self { raw }
        }

        pub fn build(&self) -> EntireWorld {
            let lines = self.raw.lines().collect::<Vec<&str>>();
            let columns = lines.first().unwrap_or(&"").len();
            let mut grounds = vec![];

            for line in &lines {
                for char in line.trim().chars() {
                    if char == '1' {
                        grounds.push(Ground::FreshWater)
                    } else {
                        grounds.push(Ground::Soil)
                    }
                }
            }

            EntireWorld::new(
                Layers::new(
                    FilledLayer::new(grounds),
                    FilledLayer::new(vec![Floor::Nothing; lines.len() * columns]),
                    CompositeLayer::new(vec![None; lines.len() * columns]),
                ),
                lines.len(),
                columns,
            )
        }
    }

    #[rstest]
    #[case(
        "00000
         01110
         01110
         01110
         00000",
         // FIXME BS NOW : algo is not finished !!! See https://gamedev.stackexchange.com/questions/207307/generate-coastal-line-from-water-tiles
         vec![vec![(1, 1), (2, 1), (3, 1), (1,2), (3,2), (1, 3), (2, 3), (3, 3)]]
    )]
    fn test_camera_world_area(#[case] map: &str, #[case] expected: Vec<Vec<(isize, isize)>>) {
        let world = WorldFromStrBuilder::new(map).build();

        let lakes = MapBuilder::new(&world).lakes();

        let expected = expected
            .iter()
            .map(|lake| {
                lake.iter()
                    .map(|(x, y)| AbsoluteWorldPoint(AbsoluteWorldRowI(*y), AbsoluteWorldColI(*x)))
                    .collect::<Vec<AbsoluteWorldPoint>>()
            })
            .collect::<Vec<Vec<AbsoluteWorldPoint>>>();
        debug_assert_eq!(lakes, expected)
    }
}
