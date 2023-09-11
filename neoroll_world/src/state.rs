use crate::{
    generator::{dummy::DummyWorldGenerator, WorldGenerator},
    space::{ColI, RegionCoordinate, RowI},
    tile::RegionTile,
};

pub const REGION_TILE_WIDTH: usize = 16;
pub const REGION_TILE_HEIGHT: usize = 16;

pub struct Region {
    tile: RegionTile,
}

#[derive(Default)]
pub struct World {
    regions: Vec<Region>,
    lines: usize,
    columns: usize,
    tile_width: usize,
    tile_height: usize,
}

impl World {
    pub fn from_random(lines: usize, columns: usize) -> Self {
        let mut world = World {
            lines,
            columns,
            tile_width: REGION_TILE_WIDTH,
            tile_height: REGION_TILE_HEIGHT,
            ..Default::default()
        };

        // Determine grass land center
        let center_lines = (lines / 10).clamp(0, 50);
        let center_columns = (columns / 10).clamp(0, 50);
        let mut center = vec![];
        for row in -(center_lines as isize / 2)..(center_lines as isize / 2) {
            for col in -(center_columns as isize / 2)..(center_columns as isize / 2) {
                center.push(RegionCoordinate(RowI(row), ColI(col)))
            }
        }

        let generator = DummyWorldGenerator::default().forced_grass_lands(center);
        for row in -(lines as isize / 2)..(lines as isize / 2) {
            for col in -(columns as isize / 2)..(columns as isize / 2) {
                world.regions.push(Region::new(
                    generator.region(&world, RegionCoordinate(RowI(row), ColI(col))),
                ))
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

    pub fn push_region(&mut self, region: Region) {
        self.regions.push(region)
    }

    pub fn region(&self, row: RowI, col: ColI) -> &Region {
        let i = row.0 * self.columns as isize + col.0;
        &self.regions[i as usize]
    }

    pub fn tile_width(&self) -> usize {
        self.tile_width
    }

    pub fn tile_height(&self) -> usize {
        self.tile_height
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
