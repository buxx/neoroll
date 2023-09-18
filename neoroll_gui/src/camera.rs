use bevy::prelude::*;
use neoroll_world::{
    map::{area::MapArea, AbsoluteMapColI, AbsoluteMapPoint, AbsoluteMapRowI, MAP_TILE_FACTOR},
    space::{area::WorldArea, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI},
};

use crate::graphics::{REGION_TILE_HEIGHT, REGION_TILE_WIDTH};

#[derive(Component, Default, Debug)]
pub struct PlayerCamera;

pub fn camera_world_area(target: UVec2, translation: Vec3, scale: Vec3) -> WorldArea {
    let width = target.x as f32 * scale.x;
    let height = target.y as f32 * scale.y;

    let start_point = Vec2::new(
        -(width / 2.) + translation.x,
        -(height / 2.) - translation.y,
    );

    let start_tile = Vec2::new(
        start_point.x / REGION_TILE_WIDTH as f32,
        start_point.y / REGION_TILE_HEIGHT as f32,
    );

    let columns = width as usize / REGION_TILE_WIDTH;
    let lines = height as usize / REGION_TILE_HEIGHT;

    WorldArea::new(
        AbsoluteWorldPoint(
            AbsoluteWorldRowI(start_tile.y as isize),
            AbsoluteWorldColI(start_tile.x as isize),
        ),
        lines,
        columns,
    )
}

pub fn camera_map_area(target: UVec2, translation: Vec3, scale: Vec3) -> MapArea {
    let width = target.x as f32 * scale.x;
    let height = target.y as f32 * scale.y;

    let start_point = Vec2::new(
        -(width / 2.) + translation.x,
        -(height / 2.) - translation.y,
    );

    let start_tile = Vec2::new(
        start_point.x / (REGION_TILE_WIDTH * MAP_TILE_FACTOR) as f32,
        start_point.y / (REGION_TILE_HEIGHT * MAP_TILE_FACTOR) as f32,
    );

    let columns = width as usize / (REGION_TILE_WIDTH * MAP_TILE_FACTOR);
    let lines = height as usize / (REGION_TILE_HEIGHT * MAP_TILE_FACTOR);

    MapArea::new(
        AbsoluteMapPoint(
            AbsoluteMapRowI(start_tile.y as isize),
            AbsoluteMapColI(start_tile.x as isize),
        ),
        lines,
        columns,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(UVec2::new(32, 32), Vec3::new(0., 0., 0.), (-1, -1), 2, 2)]
    #[case(UVec2::new(32, 32), Vec3::new(16., -16., 0.), (0, 0), 2, 2)]
    #[case(UVec2::new(64, 64), Vec3::new(0., 0., 0.), (-2, -2), 4, 4)]
    #[case(UVec2::new(64, 64), Vec3::new(32., -32., 0.), (0, 0), 4, 4)]
    #[case(UVec2::new(160, 160), Vec3::new(160., -160., 0.), (5, 5), 10, 10)]
    fn test_camera_world_area(
        #[case] target: UVec2,
        #[case] translation: Vec3,
        #[case] start: (isize, isize),
        #[case] lines: usize,
        #[case] columns: usize,
    ) {
        // TODO : Test with scale
        let area = camera_world_area(target, translation, Vec3::new(1., 1., 1.));

        assert_eq!(
            area.start(),
            AbsoluteWorldPoint(AbsoluteWorldRowI(start.0), AbsoluteWorldColI(start.1))
        );
        assert_eq!(area.lines(), lines);
        assert_eq!(area.columns(), columns);
    }
}
