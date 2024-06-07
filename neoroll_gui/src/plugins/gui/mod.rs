pub mod build;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_tileset::prelude::Tilesets;
use build::{
    display_build_cursor, display_build_outline, spawn_build_cursor, spawn_build_outline, try_build,
};
use neoroll_server::{server::ClientMessage, state::game::ClientGameMessage};
use neoroll_world::gameplay::build::Buildable;

use crate::utils::{EventReaderShortcuts, TileName};

use super::{game::GameStateWrapper, server::gateway::GatewayWrapper};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiState>()
            .add_event::<SwitchDisplayWindow>()
            .add_systems(
                Update,
                (
                    gui,
                    switch_gui_display,
                    display_build_cursor,
                    display_build_outline,
                    try_build,
                ),
            );
    }
}

pub enum Current {
    Explore,
    Build(Buildable),
}

impl Default for Current {
    fn default() -> Self {
        Self::Explore
    }
}

#[derive(Resource, Default)]
pub struct GuiState {
    current: Current,
    display_window: bool,
    server_speed_request: u8,
    is_pointer_over_area: bool,
}

impl GuiState {
    pub fn display_window(&self) -> bool {
        self.display_window
    }

    pub fn current(&self) -> &Current {
        &self.current
    }

    pub fn set_current(&mut self, current: Current) {
        self.current = current;
    }

    pub fn set_display_window(&mut self, display_window: bool) {
        self.display_window = display_window;
    }

    pub fn set_is_pointer_over_area(&mut self, is_pointer_over_area: bool) {
        self.is_pointer_over_area = is_pointer_over_area;
    }

    pub fn is_pointer_over_area(&self) -> bool {
        self.is_pointer_over_area
    }
}

#[derive(Event)]
pub struct SwitchDisplayWindow;

fn switch_gui_display(
    mut state: ResMut<GuiState>,
    mut switch_gui_display: EventReader<SwitchDisplayWindow>,
) {
    if switch_gui_display.has_been_set() {
        state.display_window = !state.display_window
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn gui(
    gateway: Res<GatewayWrapper>,
    mut state: ResMut<GuiState>,
    mut contexts: EguiContexts,
    game_state: Res<GameStateWrapper>,
    mut commands: Commands,
    tilesets: Tilesets,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if state.display_window() {
        let context = contexts.ctx_mut();

        egui::Window::new("").show(context, |ui| {
            if let Some(game) = game_state.state() {
                ui.label(&game.tribe_id().to_string());

                if game.build().can_build_campfire() && ui.button("Campfire").clicked() {
                    state.set_current(Current::Build(Buildable::Campfire));
                    state.set_display_window(false);
                    spawn_build_cursor(&mut commands, Buildable::Campfire, tilesets);
                    spawn_build_outline(&mut commands, meshes, materials);
                }

                // TODO: state.server_speed_request must be fixed by previously set value (when disconnect/reconnect)
                if ui
                    .add(
                        egui::Slider::new(&mut state.server_speed_request, 0..=100)
                            .text("Server speed"),
                    )
                    .changed()
                {
                    gateway.send(ClientMessage::Game(ClientGameMessage::RequestServerSpeed(
                        state.server_speed_request,
                    )));
                };
            }
        });

        state.set_is_pointer_over_area(context.is_pointer_over_area());
    } else {
        state.set_is_pointer_over_area(false);
    }
}

impl TileName for Buildable {
    fn tile_name(&self) -> &str {
        match self {
            Buildable::Campfire => "Campfire",
        }
    }
}
