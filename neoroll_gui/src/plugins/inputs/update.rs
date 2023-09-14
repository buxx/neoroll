use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use crate::plugins::world::container::WorldPartContainerNeedRefresh;

use super::{drag::DraggedScreen, state::InputState};

pub fn update_inputs(
    mut input_state: ResMut<InputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut world_part_container_need_change: EventWriter<WorldPartContainerNeedRefresh>,
    mut dragged_screen: EventWriter<DraggedScreen>,
) {
    let mut camera = camera.single_mut();

    // Clicks and Drags
    for event in mouse_button_input_events.iter() {
        match event.state {
            ButtonState::Pressed => {
                let point = input_state.cursor();
                input_state.start_click(event.button, point);
            }
            ButtonState::Released => {
                input_state.end_click();
            }
        }
    }

    // Motion
    for event in cursor_moved_events.iter() {
        if input_state.click().is_some() {
            let reference = input_state.cursor();
            let vector = Vec3::new(
                event.position.x - reference.x,
                event.position.y - reference.y,
                0.,
            );
            dragged_screen.send(DraggedScreen(vector));
            camera.translation.x -= vector.x;
            camera.translation.y += vector.y;
            // Avoid ugly pixels by translate only on entire pixels
            camera.translation = camera.translation.round();

            world_part_container_need_change.send(WorldPartContainerNeedRefresh)
        }

        *input_state.cursor_mut() = event.position
    }
}
