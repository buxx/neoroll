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
        gui::SwitchDisplayWindow,
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
pub fn update_inputs(
    mut input_state: ResMut<InputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_input_events: EventReader<MouseWheel>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut camera: Query<&mut Transform, (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
    mut world_container_need_refresh: EventWriter<WorldPartContainerNeedRefresh>,
    mut map_container_need_refresh: EventWriter<MapPartContainerNeedRefresh>,
    mut world_container_refreshed: EventWriter<WorldPartContainerRefreshed>,
    mut map_container_refreshed: EventWriter<MapPartContainerRefreshed>,
    mut switch_gui_display: EventWriter<SwitchDisplayWindow>,
    mut dragged_screen: EventWriter<DraggedScreen>,
    mut world_part: ResMut<WorldPartContainer>,
    mut map_part: ResMut<MapPartContainer>,
    gateway: Res<GatewayWrapper>,
) {
    let mut camera = camera.single_mut();

    // Keyboard
    for event in keyboard_events.iter() {
        if let Some(KeyCode::Space) = event.key_code {
            if let ButtonState::Released = event.state {
                switch_gui_display.send(SwitchDisplayWindow)
            }
        }
    }

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

        if AlphaByScale::world().display(camera.scale.x) {
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

        if AlphaByScale::map().display(camera.scale.x) {
            map_container_need_refresh.send(MapPartContainerNeedRefresh);
        } else {
            map_part.0.clear();
            map_container_refreshed.send(MapPartContainerRefreshed);
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
