use bevy::prelude::*;
use neoroll_world::state::{EntireWorld, NewRegions, WorldArea};

use super::container::WorldPartContainer;

#[derive(Resource, Default)]
pub struct WorldUpdater {
    // TODO : For now, store entire world here to simply develop
    pub world: Option<EntireWorld>,
}

impl WorldUpdater {
    pub fn update(&self, world_part: &mut WorldPartContainer, area: WorldArea) {
        // TODO : here will be network stuff
        if let Some(world) = &self.world {
            let current_area = world_part.0.area();
            // Send to server ;; Fake server part start
            let new_regions = NewRegions::from_world_area(world, &area, current_area);
            // Fake server part end ;; Receive from Sever
            info!("Received {} tiles", new_regions.len());
            world_part.0.switch(new_regions, area);
        }
    }
}
