use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    button: Res<Input<MouseButton>>,
) {
    let mut camera = camera.single_mut();
    if button.just_pressed(MouseButton::Left) {
        camera.translation += Vec3::new(1.0, 0.0, 1.0);
    }
}
