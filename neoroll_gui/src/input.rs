use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use crate::world::WorldPartContainerNeedRefresh;

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
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut world_part_container_need_change: EventWriter<WorldPartContainerNeedRefresh>,
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
            }
        }
    }

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
            // Avoid ugly pixels by translate only on entire pixels
            camera.translation = camera.translation.round();

            world_part_container_need_change.send(WorldPartContainerNeedRefresh)
        }

        input_state.cursor = event.position
    }
}

pub fn manual_refresh_world_part_container(
    keyboard_input: Res<Input<KeyCode>>,
    mut world_part_container_need_change: EventWriter<WorldPartContainerNeedRefresh>,
) {
    if keyboard_input.just_released(KeyCode::F5) {
        world_part_container_need_change.send(WorldPartContainerNeedRefresh)
    }
}
