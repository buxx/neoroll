use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

#[derive(Resource, Default)]
pub struct InputState {
    cursor: Vec2,
    click: Option<(MouseButton, Vec2)>,
}

impl InputState {
    pub fn cursor(&self) -> &Vec2 {
        &self.cursor
    }

    pub fn click(&self) -> &Option<(MouseButton, Vec2)> {
        &self.click
    }

    pub fn start_click(&mut self, button: MouseButton, point: Vec2) {
        self.click = Some((button, point))
    }

    pub fn end_click(&mut self) {
        self.click = None;
    }
}

pub fn inputs(
    mut input_state: ResMut<InputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut camera = camera.single_mut();

    // Clicks and Drags
    for event in mouse_button_input_events.iter() {
        match event.state {
            ButtonState::Pressed => {
                let point = input_state.cursor;
                input_state.start_click(event.button, point);
            }
            ButtonState::Released => {
                input_state.end_click();
                // Avoid ugly pixels by translate only on entire pixels
                camera.translation = camera.translation.round();
            }
        }
    }

    // // Wheel
    // for event in mouse_wheel_events.iter() {
    //     camera.scale += event.y;
    // }

    // Motion
    for event in cursor_moved_events.iter() {
        if input_state.click.is_some() {
            let reference = input_state.cursor;
            let vector = Vec3::new(
                event.position.x - reference.x,
                event.position.y - reference.y,
                0.,
            );
            camera.translation.x -= vector.x;
            camera.translation.y += vector.y;
        }

        input_state.cursor = event.position
    }
}
