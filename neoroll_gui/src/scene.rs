use bevy::prelude::*;
use neoroll_world::space::{AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI};

use crate::graphics::{REGION_TILE_HEIGHT, REGION_TILE_WIDTH};

pub struct ScenePoint {
    x: f32,
    y: f32,
}

impl ScenePoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_world_point(world_point: &AbsoluteWorldPoint) -> Self {
        Self {
            x: world_point.col_i().0 as f32 * REGION_TILE_WIDTH as f32,
            y: world_point.row_i().0 as f32 * REGION_TILE_HEIGHT as f32,
        }
    }

    pub fn apply(&self, x: f32, y: f32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn to_vec3(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, -self.y, z)
    }
}

impl From<ScenePoint> for Vec3 {
    fn from(val: ScenePoint) -> Self {
        Vec3::new(val.x, -val.y, 0.)
    }
}

impl From<ScenePoint> for Vec2 {
    fn from(val: ScenePoint) -> Self {
        Vec2::new(val.x, -val.y)
    }
}

pub trait FromScenePoint<T> {
    fn from_scene_point(value: ScenePoint) -> T;
}

impl FromScenePoint<AbsoluteWorldPoint> for AbsoluteWorldPoint {
    fn from_scene_point(value: ScenePoint) -> AbsoluteWorldPoint {
        let absolute_world_col = value.x as isize / REGION_TILE_WIDTH as isize;
        let absolute_world_row = value.y as isize / REGION_TILE_HEIGHT as isize;
        AbsoluteWorldPoint(
            AbsoluteWorldRowI(absolute_world_row),
            AbsoluteWorldColI(absolute_world_col),
        )
    }
}
