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

    pub fn rows(&self) -> Vec<AbsoluteWorldRowI> {
        (self.start().row_i().0..self.lines())
            .map(AbsoluteWorldRowI)
            .collect()
    }

    pub fn cols(&self) -> Vec<AbsoluteWorldColI> {
        (self.start().col_i().0..self.columns())
            .map(AbsoluteWorldColI)
            .collect()
    }
}

impl WorldPart {
    pub fn from_world(world: &EntireWorld, area: WorldArea) -> Self {
        let mut regions: Vec<Option<Region>> = vec![];

        println!(
            "Build WorldPart from area: start({},{}), lines({}), columns({})",
            area.start().0 .0,
            area.start().1 .0,
            area.lines(),
            area.columns()
        );

        for row in area.rows() {
            for col in area.cols() {
                regions.push(world.region(row, col).cloned());
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
        let relative_point = RelativeWorldPoint::from_absolute(point, &self.area);
        let i = relative_point.row_i().0 * self.area.columns() + relative_point.col_i().0;

        if i >= self.regions.len() {
            // println!("WARN1 : {} >= {}", i, self.regions.len());
            return &None;
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
                    AbsoluteWorldRowI(row),
                    AbsoluteWorldColI(col),
                ))
            }
        }

        let generator = DummyWorldGenerator::default().forced_grass_lands(center);
        for row in 0..lines {
            for col in 0..columns {
                let point = AbsoluteWorldPoint(AbsoluteWorldRowI(row), AbsoluteWorldColI(col));
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

    pub fn region(&self, row: AbsoluteWorldRowI, col: AbsoluteWorldColI) -> Option<&Region> {
        let i = row.0 * self.columns + col.0;

        if i >= self.regions.len() {
            // println!("WARN2 : {} >= {}", i, self.regions.len());
            return None;
        }

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
