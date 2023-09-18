use bevy::prelude::*;
use neoroll_world::map::{area::MapArea, patch::NewSectors, Map};

use super::container::MapPartContainer;

#[derive(Resource, Default)]
pub struct MapUpdater {
    // TODO : For now, store entire world here to simply develop
    pub map: Option<Map>,
}

impl MapUpdater {
    pub fn update(&self, map_part: &mut MapPartContainer, area: MapArea) {
        // TODO : here will be network stuff
        if let Some(map) = &self.map {
            let current_area = map_part.0.area();
            // Send to server ;; Fake server part start
            let new_sectors = NewSectors::from_map_area(map, &area, current_area);
            // Fake server part end ;; Receive from Sever
            info!("Received {} sectors", new_sectors.len());
            map_part.0.switch(new_sectors, area);
        }
    }
}
