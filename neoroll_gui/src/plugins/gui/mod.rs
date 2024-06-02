use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::utils::EventReaderShortcuts;

use super::game::GameStateWrapper;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiState>()
            .add_event::<SwitchGuiDisplay>()
            .add_systems(Update, (gui_display, switch_gui_display));
    }
}

#[derive(Resource, Default)]
pub struct GuiState {
    gui: bool,
}

impl GuiState {
    pub fn gui(&self) -> bool {
        self.gui
    }
}

#[derive(Event)]
pub struct SwitchGuiDisplay;

fn switch_gui_display(
    mut state: ResMut<GuiState>,
    mut switch_gui_display: EventReader<SwitchGuiDisplay>,
) {
    if switch_gui_display.has_been_set() {
        state.gui = !state.gui
    }
}

fn gui_display(
    state: Res<GuiState>,
    mut contexts: EguiContexts,
    game_state: Res<GameStateWrapper>,
) {
    if state.gui() {
        egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            if let Some(state) = game_state.state() {
                ui.label(&state.tribe_id().to_string());
            }
        });
    }
}
