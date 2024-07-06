use bevy_egui::egui::{self, Ui};

use super::GuiAction;
use neoroll_server::{server::ClientMessage, state::game::ClientGameMessage};

use super::paint::Painter;

impl<'a> Painter<'a> {
    pub fn root(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        ui.label(
            "Establish your tribe by building a campfire.
 Then, ensure your tribesmen food and supplies by setting targets."
        );

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
