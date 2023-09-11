use bevy::prelude::*;

use crate::world::WorldContainer;

#[derive(Component, Default, Debug)]
pub struct PlayerCamera;

pub fn debug_camera(
    keyboard_input: Res<Input<KeyCode>>,
    player_camera: Query<(&PlayerCamera, &Camera, &mut Transform)>,
    world: Res<WorldContainer>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        let (_, camera, transform) = player_camera.single();

        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;

        let start_pixel = Vec2::new(
            -(target.x as f32 / 2.) + translation.x,
            -(target.y as f32 / 2.) - translation.y,
        );
        let end_pixel = Vec2::new(
            start_pixel.x + target.x as f32,
            start_pixel.y + target.y as f32,
        );

        let start_tile = Vec2::new(
            start_pixel.x / world.0.tile_width() as f32,
            start_pixel.y / world.0.tile_height() as f32,
        );
        let end_tile = Vec2::new(
            end_pixel.x / world.0.tile_width() as f32,
            end_pixel.y / world.0.tile_height() as f32,
        );

        info!("start({:?}), end({:?})", start_tile, end_tile);
    }
}
