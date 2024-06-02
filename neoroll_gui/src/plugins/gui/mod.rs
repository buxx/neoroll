pub mod build;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_tileset::prelude::Tilesets;
use build::{display_build_cursor, spawn_display_cursor};
use neoroll_world::gameplay::build::Buildable;

use crate::utils::{EventReaderShortcuts, TileName};

use super::game::GameStateWrapper;

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
                    display_build_cursor,
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

fn gui(
    mut state: ResMut<GuiState>,
    mut contexts: EguiContexts,
    game_state: Res<GameStateWrapper>,
    commands: Commands,
    tilesets: Tilesets,
) {
    if state.display_window() {
        egui::Window::new("").show(contexts.ctx_mut(), |ui| {
            if let Some(game) = game_state.state() {
                ui.label(&game.tribe_id().to_string());

                if game.build().can_build_campfire() && ui.button("Campfire").clicked() {
                    state.set_current(Current::Build(Buildable::Campfire));
                    state.set_display_window(false);
                    spawn_display_cursor(commands, Buildable::Campfire, tilesets);
                }
            }
        });
    }
}

impl TileName for Buildable {
    fn tile_name(&self) -> &str {
        match self {
            Buildable::Campfire => "Campfire",
        }
    }
}
