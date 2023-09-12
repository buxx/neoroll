use bevy::prelude::*;
use neoroll_world::{
    space::{AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI},
    state::{WorldArea, REGION_TILE_HEIGHT, REGION_TILE_WIDTH},
};

#[derive(Component, Default, Debug)]
pub struct PlayerCamera;

pub fn debug_camera(
    keyboard_input: Res<Input<KeyCode>>,
    player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::F12) {
        let (_, camera, transform) = player_camera.single();

        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let area = camera_world_area(target, translation);

        info!("area({:?})", area);
    }
}

pub fn camera_world_area(target: UVec2, translation: Vec3) -> WorldArea {
    let start_pixel = Vec2::new(
        -(target.x as f32 / 2.) + translation.x,
        -(target.y as f32 / 2.) - translation.y,
    );
    let end_pixel = Vec2::new(
        start_pixel.x + target.x as f32,
        start_pixel.y + target.y as f32,
    );

    let start_tile = Vec2::new(
        start_pixel.x / REGION_TILE_WIDTH as f32,
        start_pixel.y / REGION_TILE_HEIGHT as f32,
    );
    let end_tile = Vec2::new(
        end_pixel.x / REGION_TILE_WIDTH as f32,
        end_pixel.y / REGION_TILE_HEIGHT as f32,
    );

    WorldArea::new(
        AbsoluteWorldPoint(
            AbsoluteWorldRowI(start_tile.y as usize),
            AbsoluteWorldColI(start_tile.x as usize),
        ),
        end_tile.y as usize - start_tile.y as usize,
        end_tile.x as usize - start_tile.x as usize,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(UVec2::new(32, 32), Vec3::new(0., 0., 0.), (0, 0), 1, 1)]
    #[case(UVec2::new(32, 32), Vec3::new(16., -16., 0.), (0, 0), 2, 2)]
    #[case(UVec2::new(64, 64), Vec3::new(0., 0., 0.), (0, 0), 2, 2)]
    #[case(UVec2::new(64, 64), Vec3::new(32., -32., 0.), (0, 0), 4, 4)]
    #[case(UVec2::new(160, 160), Vec3::new(160., -160., 0.), (5, 5), 10, 10)]
    fn test_camera_world_area(
        #[case] target: UVec2,
        #[case] translation: Vec3,
        #[case] start: (usize, usize),
        #[case] lines: usize,
        #[case] columns: usize,
    ) {
        let area = camera_world_area(target, translation);

        assert_eq!(
            area.start(),
            AbsoluteWorldPoint(AbsoluteWorldRowI(start.0), AbsoluteWorldColI(start.1))
        );
        assert_eq!(area.lines(), lines);
        assert_eq!(area.columns(), columns);
    }
}
