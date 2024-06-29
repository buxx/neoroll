use bevy_egui::egui::{self, Ui};
use neoroll_world::gameplay::build::Buildable;

use super::GuiAction;
use neoroll_server::{server::ClientMessage, state::game::ClientGameMessage};

use super::paint::Painter;

impl<'a> Painter<'a> {
    pub fn root(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        ui.label(&self.game().tribe_id().to_string());

        if self.game().build().can_build_campfire() && ui.button("Campfire").clicked() {
            return vec![GuiAction::Build(Buildable::Campfire)];
        }

        if self.game().build().can_build_storage() && ui.button("Storage").clicked() {
            return vec![GuiAction::Build(Buildable::Storage)];
        }

        ui.label(&format!("{:?}", self.game().materials().total()));

        // TODO: self.state.server_speed_request must be fixed by previously set value (when disconnect/reconnect)
        if ui
            .add(
                egui::Slider::new(self.state_mut().server_speed_request_mut(), 0..=200)
                    .text("Server speed"),
            )
            .changed()
        {
            self.gateway()
                .send(ClientMessage::Game(ClientGameMessage::RequestServerSpeed(
                    self.state().server_speed_request(),
                )));
        };

        vec![]
    }
}
