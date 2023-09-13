use crate::{
    generator::{dummy::DummyWorldGenerator, WorldGenerator},
    space::{AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI, RelativeWorldPoint},
    tile::RegionTile,
};

pub const REGION_TILE_WIDTH: usize = 16;
pub const REGION_TILE_HEIGHT: usize = 16;

#[derive(Clone, Debug)]
pub struct Region {
    tile: RegionTile,
}

#[derive(Default)]
pub struct WorldPart {
    regions: Vec<Option<Region>>,
    area: WorldArea,
}

#[derive(Default)]
pub struct EntireWorld {
    regions: Vec<Region>,
    lines: usize,
    columns: usize,
}

#[derive(Debug, Default, Clone)]
pub struct WorldArea {
    start: AbsoluteWorldPoint,
    lines: usize,
    columns: usize,
}

impl WorldArea {
    pub fn new(start: AbsoluteWorldPoint, lines: usize, columns: usize) -> Self {
        Self {
            start,
            lines,
            columns,
        }
    }

    pub fn start(&self) -> AbsoluteWorldPoint {
        self.start
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn start_row(&self) -> AbsoluteWorldRowI {
        AbsoluteWorldRowI(self.start().row_i().0)
    }

    pub fn end_row(&self) -> AbsoluteWorldRowI {
        AbsoluteWorldRowI(self.start_row().0 + self.lines() as isize)
    }

    pub fn start_col(&self) -> AbsoluteWorldColI {
        AbsoluteWorldColI(self.start().col_i().0)
    }

    pub fn end_col(&self) -> AbsoluteWorldColI {
        AbsoluteWorldColI(self.start_col().0 + self.columns() as isize)
    }

    pub fn rows(&self) -> Vec<AbsoluteWorldRowI> {
        let from = self.start_row().0;
        let to = self.end_row().0;
        (from..to).map(AbsoluteWorldRowI).collect()
    }

    pub fn cols(&self) -> Vec<AbsoluteWorldColI> {
        let from = self.start_col().0;
        let to = self.end_col().0;
        (from..to).map(AbsoluteWorldColI).collect()
    }
}

impl WorldPart {
    pub fn from_world(world: &EntireWorld, area: WorldArea) -> Self {
        let mut regions: Vec<Option<Region>> = vec![];

        for row in area.rows() {
            for col in area.cols() {
                regions.push(world.region(AbsoluteWorldPoint(row, col)).cloned());
            }
        }

        Self { regions, area }
    }

    pub fn regions(&self) -> Vec<(AbsoluteWorldPoint, &Option<Region>)> {
        let mut regions = vec![];

        for row in self.area().rows() {
            for col in self.area().cols() {
                let point = AbsoluteWorldPoint(row, col);
                let region = self.region(point);
                regions.push((point, region));
            }
        }

        regions
    }

    pub fn region(&self, point: AbsoluteWorldPoint) -> &Option<Region> {
        // Outside
        if point.row_i().0 >= self.area.end_row().0
            || point.row_i().0 < self.area.start_row().0
            || point.col_i().0 >= self.area.end_col().0
            || point.col_i().0 < self.area.start_col().0
        {
            return &None;
        }

        let relative_point = RelativeWorldPoint::from_absolute(point, &self.area);
        let row_i = relative_point.row_i().0;
        let col_i = relative_point.col_i().0;
        assert!(row_i >= 0);
        assert!(col_i >= 0);
        let row_i = row_i as usize;
        let col_i = col_i as usize;

        let i = row_i * self.area.columns() + col_i;
        if i >= self.regions.len() {
            println!("oups");
        }
        &self.regions[i]
    }

    pub fn area(&self) -> &WorldArea {
        &self.area
    }
}

impl EntireWorld {
    pub fn from_random(lines: usize, columns: usize) -> Self {
        let mut world = EntireWorld {
            lines,
            columns,
            ..Default::default()
        };

        // Determine grass land center
        let center_row = lines / 2;
        let center_col = columns / 2;
        let width = 5;
        let mut center = vec![];
        for row in (center_row - width)..(center_row + width) {
            for col in (center_col - width)..(center_col + width) {
                center.push(AbsoluteWorldPoint(
                    AbsoluteWorldRowI(row as isize),
                    AbsoluteWorldColI(col as isize),
                ))
            }
        }

        let generator = DummyWorldGenerator::default().forced_grass_lands(center);
        for row in 0..lines {
            for col in 0..columns {
                let point = AbsoluteWorldPoint(
                    AbsoluteWorldRowI(row as isize),
                    AbsoluteWorldColI(col as isize),
                );
                let new_tile = generator.region(&world, point);
                let new_region = Region::new(new_tile);
                world.regions.push(new_region);
            }
        }

        world
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn region(&self, point: AbsoluteWorldPoint) -> Option<&Region> {
        // Outside
        if point.row_i().0 >= self.lines as isize
            || point.row_i().0 < 0
            || point.col_i().0 >= self.columns as isize
            || point.col_i().0 < 0
        {
            return None;
        }

        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;

        let i = row_i * self.columns + col_i;

        Some(&self.regions[i])
    }

    pub fn regions(&self) -> &[Region] {
        self.regions.as_ref()
    }
}

impl Region {
    pub fn new(tile: RegionTile) -> Self {
        Self { tile }
    }

    pub fn tile(&self) -> &RegionTile {
        &self.tile
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[cfg(test)]
    #[fixture]
    fn entire_world() -> EntireWorld {
        let lines = 5;
        let columns = 5;
        let regions = (0..25)
            .map(|_| Region::new(RegionTile::GrassLand))
            .collect();
        EntireWorld {
            regions,
            lines,
            columns,
        }
    }

    #[rstest]
    #[case(
        (0, 0),
        5,
        5,
        vec![
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
        ]
    )]
    #[case(
        (0, 0),
        2,
        5,
        vec![
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
        ]
    )]
    #[case(
        (0, 0),
        2,
        2,
        vec![
            1, 1,
            1, 1,
        ]
    )]
    #[case(
        (0, 0),
        6,
        6,
        vec![
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0,
        ]
    )]
    #[case(
        (-1, -1),
        5,
        5,
        vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 1,
            0, 1, 1, 1, 1,
            0, 1, 1, 1, 1,
            0, 1, 1, 1, 1,
        ]
    )]
    fn test_world_part(
        entire_world: EntireWorld,
        #[case] start: (isize, isize),
        #[case] lines: usize,
        #[case] columns: usize,
        #[case] expected: Vec<usize>,
    ) {
        let (start_row, start_col) = start;
        let area = WorldArea::new(
            AbsoluteWorldPoint(AbsoluteWorldRowI(start_row), AbsoluteWorldColI(start_col)),
            lines,
            columns,
        );

        let world_part = WorldPart::from_world(&entire_world, area);

        let regions: Vec<usize> = world_part
            .regions()
            .iter()
            .map(|(_, r)| r.is_some() as usize)
            .collect();
        assert_eq!(regions, expected)
    }
}
