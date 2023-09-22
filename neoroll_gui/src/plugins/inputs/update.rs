use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseWheel},
        ButtonState,
    },
    prelude::*,
};

use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    plugins::{
        map::container::MapPartContainerNeedRefresh,
        world::container::WorldPartContainerNeedRefresh,
    },
};

use super::{drag::DraggedScreen, state::InputState};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn update_inputs(
    mut input_state: ResMut<InputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_input_events: EventReader<MouseWheel>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera: Query<&mut Transform, (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
    mut world_container_need_refresh: EventWriter<WorldPartContainerNeedRefresh>,
    mut map_container_need_refresh: EventWriter<MapPartContainerNeedRefresh>,
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

    // Wheel
    for event in mouse_wheel_input_events.iter() {
        camera.scale -= event.y / 5.;
        camera.scale.x = camera.scale.x.clamp(0.25, 16.);
        camera.scale.y = camera.scale.y.clamp(0.25, 16.);
        world_container_need_refresh.send(WorldPartContainerNeedRefresh);
        map_container_need_refresh.send(MapPartContainerNeedRefresh);
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
            camera.translation.x -= vector.x * camera.scale.x;
            camera.translation.y += vector.y * camera.scale.y;
            // Avoid ugly pixels by translate only on entire pixels
            camera.translation = camera.translation.round();

            world_container_need_refresh.send(WorldPartContainerNeedRefresh);
            map_container_need_refresh.send(MapPartContainerNeedRefresh);
        }

        *input_state.cursor_mut() = event.position
    }
}
