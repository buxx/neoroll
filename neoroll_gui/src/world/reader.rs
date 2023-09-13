use bevy::prelude::*;
use neoroll_world::state::{EntireWorld, WorldArea, WorldPart};

use super::container::WorldPartContainer;

#[derive(Resource, Default)]
pub struct WorldReader {
    // TODO : For now, store entire world here to simply develop
    pub world: Option<EntireWorld>,
}

impl WorldReader {
    pub fn update(&self, world_part: &mut WorldPartContainer, area: WorldArea) {
        // TODO : here will be network stuff
        if let Some(world) = &self.world {
            // TODO : Instead recreate entire part, update current by grabbing new tiles, and removing not required enough
            world_part.0 = WorldPart::from_world(world, area);
        }
    }
}
