use self::{area::MapArea, sector::Sector};

pub mod area;
pub mod builder;
pub mod element;
pub mod part;
pub mod patch;
pub mod sector;

pub const MAP_TILE_FACTOR: usize = 32;

pub struct Map {
    sectors: Vec<Sector>,
    lines: usize,
    columns: usize,
}

impl Map {
    pub fn empty() -> Map {
        Map {
            sectors: vec![],
            lines: 0,
            columns: 0,
        }
    }

    fn new(sectors: Vec<Sector>, lines: usize, columns: usize) -> Map {
        Map {
            sectors,
            lines,
            columns,
        }
    }

    pub fn sector(&self, point: &AbsoluteMapPoint) -> Option<&Sector> {
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
        Some(&self.sectors[i])
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct AbsoluteMapRowI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct AbsoluteMapColI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct AbsoluteMapPoint(pub AbsoluteMapRowI, pub AbsoluteMapColI);

impl AbsoluteMapPoint {
    pub fn zero() -> Self {
        Self(AbsoluteMapRowI(0), AbsoluteMapColI(0))
    }

    pub fn row_i(&self) -> &AbsoluteMapRowI {
        &self.0
    }

    pub fn col_i(&self) -> &AbsoluteMapColI {
        &self.1
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct RelativeMapRowI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct RelativeMapColI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct RelativeMapPoint(pub RelativeMapRowI, pub RelativeMapColI);

impl RelativeMapPoint {
    pub fn row_i(&self) -> &RelativeMapRowI {
        &self.0
    }

    pub fn col_i(&self) -> &RelativeMapColI {
        &self.1
    }

    pub fn from_absolute(point: &AbsoluteMapPoint, reference: &MapArea) -> Self {
        Self(
            RelativeMapRowI(point.row_i().0 - reference.start().row_i().0),
            RelativeMapColI(point.col_i().0 - reference.start().col_i().0),
        )
    }
}
