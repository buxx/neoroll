pub mod keep_stock;

use bevy_egui::egui::{Grid, Ui};
use neoroll_world::gameplay::target::{Target, TargetId};

use super::GuiAction;

use super::paint::Painter;

impl<'a> Painter<'a> {
    pub fn targets(&self, ui: &mut Ui) -> Vec<GuiAction> {
        let mut actions = vec![];

        actions.extend(self.active_targets(ui));

        actions
    }

    fn active_targets(&self, ui: &mut Ui) -> Vec<GuiAction> {
        let mut actions = vec![];

        Grid::new("targets").show(ui, |ui| {
            for target in self.game().target().targets() {
                actions.extend(self.target_row(ui, target));
                ui.end_row();
            }
        });

        actions
    }

    fn target_row(&self, ui: &mut Ui, target: (&TargetId, &Target)) -> Vec<GuiAction> {
        let mut actions = vec![];

        ui.horizontal(|ui| {
            ui.label(target.1.name());
        });

        match target.1 {
            Target::KeepStock(_, _) => {
                actions.extend(self.keep_stock_resume(ui, target));
                actions.extend(self.keep_stock_settings(ui, target));
            }
        }

        actions
    }
}
