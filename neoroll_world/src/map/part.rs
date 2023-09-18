use crate::map::RelativeMapPoint;

use super::{area::MapArea, patch::NewSectors, sector::Sector, AbsoluteMapPoint};

pub struct MapPart {
    sectors: Vec<Option<Sector>>,
    area: MapArea,
}

impl MapPart {
    pub fn new(sectors: Vec<Option<Sector>>, area: MapArea) -> Self {
        Self { sectors, area }
    }

    pub fn empty() -> Self {
        Self::new(vec![], MapArea::zero())
    }

    fn index(&self, point: &AbsoluteMapPoint) -> usize {
        let relative_point = RelativeMapPoint::from_absolute(point, &self.area);
        let row_i = relative_point.row_i().0;
        let col_i = relative_point.col_i().0;
        assert!(row_i >= 0);
        assert!(col_i >= 0);
        let row_i = row_i as usize;
        let col_i = col_i as usize;

        let i = row_i * self.area.columns() + col_i;
        assert!(i < self.sectors.len());
        i
    }

    pub fn sectors(&self) -> Vec<(AbsoluteMapPoint, &Option<Sector>)> {
        let mut sectors = vec![];

        for point in self.area().points() {
            sectors.push((point, self.sector(&point)));
        }

        sectors
    }

    pub fn sector(&self, point: &AbsoluteMapPoint) -> &Option<Sector> {
        // Outside
        if !self.area.contains(point) {
            return &None;
        }

        &self.sectors[self.index(point)]
    }

    pub fn area(&self) -> &MapArea {
        &self.area
    }

    pub fn switch(&mut self, new: NewSectors, area: MapArea) {
        let mut sectors = vec![];

        for point in area.points() {
            sectors.push(
                self.sector(&point)
                    .clone()
                    .or_else(|| new.sector(&point).cloned()),
            );
        }

        self.sectors = sectors;
        self.area = area;
    }
}
