use bevy::prelude::*;
use neoroll_world::{
    space::AbsoluteWorldPoint,
    state::{REGION_TILE_HEIGHT, REGION_TILE_WIDTH},
};

pub struct ScenePoint {
    x: f32,
    y: f32,
}

impl ScenePoint {
    pub fn from_world_point(world_point: AbsoluteWorldPoint) -> Self {
        Self {
            x: world_point.col_i().0 as f32 * REGION_TILE_WIDTH as f32,
            y: world_point.row_i().0 as f32 * REGION_TILE_HEIGHT as f32,
        }
    }
}

impl From<ScenePoint> for Vec3 {
    fn from(val: ScenePoint) -> Self {
        Vec3::new(val.x, -val.y, 0.)
    }
}
