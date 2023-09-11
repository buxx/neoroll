use crate::{
    generator::{dummy::DummyWorldGenerator, WorldGenerator},
    space::{ColI, RegionCoordinate, RowI},
    tile::RegionTile,
};

pub struct Region {
    tile: RegionTile,
}

#[derive(Default)]
pub struct World {
    regions: Vec<Region>,
    lines: usize,
    columns: usize,
}

impl World {
    pub fn from_random(lines: usize, columns: usize) -> Self {
        let mut world = World {
            lines,
            columns,
            ..Default::default()
        };

        for col in 0..columns {
            for row in 0..lines {
                world.regions.push(Region::new(
                    DummyWorldGenerator.region(&world, RegionCoordinate(RowI(row), ColI(col))),
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
        let i = row.0 * self.columns + col.0;
        &self.regions[i]
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
