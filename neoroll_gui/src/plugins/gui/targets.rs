
use bevy_egui::egui::Ui;

use super::GuiAction;

use super::paint::Painter;

impl<'a> Painter<'a> {
    pub fn targets(&mut self, _ui: &mut Ui) -> Vec<GuiAction> {


        vec![]
    }
}
