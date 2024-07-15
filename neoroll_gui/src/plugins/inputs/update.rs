use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseWheel},
        ButtonState,
    },
    prelude::*,
};
use neoroll_server::{server::ClientMessage, subscriptions::SubscriptionsMessage};

use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    graphics::AlphaByScale,
    plugins::{
        gui::{state::GuiState, Panel, SwitchDisplayWindow},
        map::container::{
            MapPartContainer, MapPartContainerNeedRefresh, MapPartContainerRefreshed,
        },
        server::gateway::GatewayWrapper,
        world::container::{
            WorldPartContainer, WorldPartContainerNeedRefresh, WorldPartContainerRefreshed,
        },
    },
};

use super::{drag::DraggedScreen, state::InputState};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn update_keyboard(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut switch_gui_display: EventWriter<SwitchDisplayWindow>,
) {
    // Keyboard
    for event in keyboard_events.iter() {
        if let Some(KeyCode::Space) = event.key_code {
            if let ButtonState::Released = event.state {
                switch_gui_display.send(SwitchDisplayWindow(Panel::Root))
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn update_clicks(
    mut input_state: ResMut<InputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    gui: Res<GuiState>,
) {
    // Clicks and Drags
    input_state.reset_clicked();
    for event in mouse_button_input_events.iter() {
        let point = input_state.cursor();
        match event.state {
            ButtonState::Pressed => {
                input_state.start_clicking(event.button, point);
            }
            ButtonState::Released => {
                input_state.end_clicking(event.button, point, gui.is_pointer_over_area());
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn update_wheel(
    window: Query<&Window>,
    mut mouse_wheel_input_events: EventReader<MouseWheel>,
    mut camera: Query<
        (&Camera, &mut Transform, &GlobalTransform),
        (With<SceneItemsCamera>, Without<BackgroundCamera>),
    >,
    mut world_container_need_refresh: EventWriter<WorldPartContainerNeedRefresh>,
    mut map_container_need_refresh: EventWriter<MapPartContainerNeedRefresh>,
    mut world_container_refreshed: EventWriter<WorldPartContainerRefreshed>,
    mut map_container_refreshed: EventWriter<MapPartContainerRefreshed>,
    mut world_part: ResMut<WorldPartContainer>,
    mut map_part: ResMut<MapPartContainer>,
    gateway: Res<GatewayWrapper>,
) {
    let window = window.single();
    let (camera, mut camera_transform, camera_global_transform) = camera.single_mut();

    // Wheel
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(world_position) =
            camera.viewport_to_world_2d(camera_global_transform, cursor_position)
        {
            for event in mouse_wheel_input_events.iter() {
                let previous_scale_x = camera_transform.scale.x;
                let previous_scale_y = camera_transform.scale.y;

                camera_transform.scale -= event.y / 5.;
                camera_transform.scale.x = camera_transform.scale.x.clamp(0.25, 16.);
                camera_transform.scale.y = camera_transform.scale.y.clamp(0.25, 16.);

                if true {
                    // This is the position of cursor from the center of the screen
                    let cursor_position_from_screen_center = Vec2::new(
                        cursor_position.x - (window.width() / 2.),
                        cursor_position.y - (window.height() / 2.),
                    );
                    // This is the offset between screen center and cursor in world 2d
                    let world_offset_from_cursor_before = Vec2::new(
                        cursor_position_from_screen_center.x * previous_scale_x,
                        cursor_position_from_screen_center.y * previous_scale_y,
                    );
                    let world_offset_from_cursor_after = Vec2::new(
                        cursor_position_from_screen_center.x * camera_transform.scale.x,
                        cursor_position_from_screen_center.y * camera_transform.scale.y,
                    );

                    let decal = Vec2::new(
                        world_offset_from_cursor_before.x - world_offset_from_cursor_after.x,
                        world_offset_from_cursor_before.y - world_offset_from_cursor_after.y,
                    );

                    camera_transform.translation = Vec3::new(
                        camera_transform.translation.x + decal.x,
                        camera_transform.translation.y - decal.y,
                        0.,
                    );
                }

                if AlphaByScale::world().display(camera_transform.scale.x) {
                    world_container_need_refresh.send(WorldPartContainerNeedRefresh);
                // If world is not anymore displayed, remove all related to World
                } else {
                    gateway.send(ClientMessage::Subscriptions(SubscriptionsMessage::SetArea(
                        None,
                    )));
                    gateway.send(ClientMessage::Subscriptions(
                        SubscriptionsMessage::SetCreatures(vec![]),
                    ));

                    world_part.0.clear();
                    world_container_refreshed.send(WorldPartContainerRefreshed);
                }

                if AlphaByScale::map().display(camera_transform.scale.x) {
                    map_container_need_refresh.send(MapPartContainerNeedRefresh);
                } else {
                    map_part.0.clear();
                    map_container_refreshed.send(MapPartContainerRefreshed);
                }
            }
        }
    }
}
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn update_motion(
    mut input_state: ResMut<InputState>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera: Query<&mut Transform, (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
    mut world_container_need_refresh: EventWriter<WorldPartContainerNeedRefresh>,
    mut map_container_need_refresh: EventWriter<MapPartContainerNeedRefresh>,
    mut dragged_screen: EventWriter<DraggedScreen>,
    gui: Res<GuiState>,
) {
    let mut camera = camera.single_mut();

    // Motion
    for event in cursor_moved_events.iter() {
        if !gui.is_pointer_over_area() && input_state.clicking().is_some() {
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
