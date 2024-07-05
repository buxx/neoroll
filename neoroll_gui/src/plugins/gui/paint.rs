use crate::plugins::{server::gateway::GatewayWrapper, world::illustration::IntoIllustration};
use bevy_egui::egui::{self, ComboBox, Ui};
use neoroll_world::space::part::WorldPart;
use strum::IntoEnumIterator;

use super::{
    state::{GuiState, GuiZoom},
    GuiAction, Panel,
};
use neoroll_server::state::client::ClientGameState;

pub struct Painter<'a> {
    game: &'a ClientGameState,
    state: &'a mut GuiState,
    world: &'a WorldPart,
    gateway: &'a GatewayWrapper,
}

impl<'a> Painter<'a> {
    pub fn new(
        game: &'a ClientGameState,
        state: &'a mut GuiState,
        world: &'a WorldPart,
        gateway: &'a GatewayWrapper,
    ) -> Self {
        Self {
            game,
            state,
            world,
            gateway,
        }
    }

    pub fn paint(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        self.header(ui);

        match self.state.current_panel() {
            Panel::Root => self.root(ui),
            Panel::Targets => self.targets(ui),
            Panel::Details => self.details(ui),
            Panel::Build => self.builds(ui),
            Panel::Stock => self.stocks(ui),
        }
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            for panel in Panel::iter() {
                let text = panel.to_string();
                ui.selectable_value(self.state.current_panel_mut(), panel, &text);
            }

            ui.separator();

            let gui_zoom = self.state_mut().zoom_mut();
            ComboBox::from_id_source("zoom")
                .selected_text("Zoom")
                .show_ui(ui, |ui| {
                    ui.selectable_value(gui_zoom, GuiZoom::Normal, "Normal");
                    ui.selectable_value(gui_zoom, GuiZoom::Big, "Big");
                    ui.selectable_value(gui_zoom, GuiZoom::VeryBig, "VeryBig");
                });

            if ui.button("Close").clicked() {
                *self.state_mut().display_window_mut() = false;
            }
        });
    }

    pub fn game(&self) -> &ClientGameState {
        self.game
    }

    pub fn state(&self) -> &GuiState {
        self.state
    }

    pub fn gateway(&self) -> &GatewayWrapper {
        self.gateway
    }

    pub fn state_mut(&mut self) -> &mut GuiState {
        self.state
    }

    pub fn world(&self) -> &WorldPart {
        self.world
    }

    pub fn illustration(&self, ui: &mut Ui, source: &dyn IntoIllustration) {
        if let Some(illustration) = source.illustration() {
            ui.add(
                egui::Image::new(illustration.data())
                    .rounding(5.0)
                    .max_height(75.),
            );
        } else {
            ui.label("");
        }
    }
}
