use std::collections::HashMap;

use super::{area::MapArea, sector::Sector, AbsoluteMapPoint, Map};

#[derive(Debug, Clone)]
pub struct NewSectors {
    sectors: HashMap<AbsoluteMapPoint, Sector>,
}

impl NewSectors {
    pub fn from_map_area(map: &Map, area: &MapArea, ignore: &MapArea) -> Self {
        let mut sectors = HashMap::new();

        for point in area.points() {
            if !ignore.contains(&point) {
                if let Some(sector) = map.sector(&point) {
                    sectors.insert(point, sector.clone());
                }
            }
        }

        Self { sectors }
    }

    pub fn sector(&self, point: &AbsoluteMapPoint) -> Option<&Sector> {
        self.sectors.get(point)
    }

    pub fn len(&self) -> usize {
        self.sectors.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
