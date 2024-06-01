use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::game::GameState;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gui_display);
    }
}

fn gui_display(state: Res<GameState>, mut contexts: EguiContexts) {
    if state.gui() {
        egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            ui.label("world");
        });
    }
}
