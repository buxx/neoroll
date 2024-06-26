use bevy_egui::egui::{Slider, Ui};
use neoroll_server::state::game::settings::TargetSetting;
use neoroll_world::gameplay::{
    target::{ComputedTarget, Target, TargetQuantity},
    Quantity,
};

use crate::plugins::gui::{paint::Painter, GuiAction, TargetAction};

impl<'a> Painter<'a> {
    pub fn keep_stock_resume(&self, ui: &mut Ui, target: &ComputedTarget) -> Vec<GuiAction> {
        ui.vertical(|ui| {
            let state_value = target.state_string();
            ui.label(&format!("State: {}", state_value));

            ui.label(&format!("Affected: {}", target.affected()));

            if !target.covered() {
                if let Some(waiting) = self.game().target().waitings().get(target.id()) {
                    let waiting_str = waiting
                        .iter()
                        .map(|w| w.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    ui.label(&format!("Require: {}", waiting_str));
                }
            };
        });

        vec![]
    }

    // FIXME BS NOW: gui send it several times (one by frame)
    pub fn keep_stock_settings(&self, ui: &mut Ui, target: &ComputedTarget) -> Vec<GuiAction> {
        match target.target() {
            Target::KeepStock(material, quantity) => match quantity {
                // TODO: choice of target quantity type
                TargetQuantity::Fixed(quantity) => {
                    let mut value = quantity.0;
                    // TODO: range by target
                    if ui.add(Slider::new(&mut value, 0..=100000)).changed() {
                        let new_target =
                            Target::KeepStock(*material, TargetQuantity::Fixed(Quantity(value)));
                        return vec![GuiAction::Target(
                            *target.id(),
                            TargetAction::Set(TargetSetting::new(new_target, target.priority())),
                        )];
                    }
                }
                TargetQuantity::PerHuman(quantity) => {
                    let mut value = quantity.0;
                    // TODO: range by target
                    if ui.add(Slider::new(&mut value, 0..=100000)).changed() {
                        let new_target =
                            Target::KeepStock(*material, TargetQuantity::PerHuman(Quantity(value)));
                        return vec![GuiAction::Target(
                            *target.id(),
                            TargetAction::Set(TargetSetting::new(new_target, target.priority())),
                        )];
                    }
                }
            },
        };

        vec![]
    }
}
