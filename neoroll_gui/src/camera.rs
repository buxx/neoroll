use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    button: Res<Input<MouseButton>>,
) {
    let mut camera = camera.single_mut();
    if button.just_pressed(MouseButton::Left) {
        println!("move camera");
        camera.translation = camera.translation.lerp(Vec3::new(1.0, 1.0, 1.0), 0.2);
    }
}
