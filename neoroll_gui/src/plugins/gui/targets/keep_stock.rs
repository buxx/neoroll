use bevy_egui::egui::{Slider, Ui};
use neoroll_world::gameplay::{
    target::{ComputedTarget, Target, TargetQuantity},
    Quantity,
};

use crate::plugins::gui::{paint::Painter, GuiAction, TargetAction};

impl<'a> Painter<'a> {
    pub fn keep_stock_resume(&self, ui: &mut Ui, _target: &ComputedTarget) -> Vec<GuiAction> {
        ui.label("TODO");

        vec![]
    }

    pub fn keep_stock_settings(&self, ui: &mut Ui, target: &ComputedTarget) -> Vec<GuiAction> {
        match target.target() {
            Target::KeepStock(material, quantity) => match quantity {
                // TODO: choice of target quantity type
                TargetQuantity::Fixed(_) => todo!(),
                TargetQuantity::PerHuman(quantity_value) => {
                    let mut value = quantity_value.0;
                    // TODO: range by target
                    if ui.add(Slider::new(&mut value, 0..=100000)).changed() {
                        let new_target =
                            Target::KeepStock(*material, TargetQuantity::PerHuman(Quantity(value)));
                        return vec![GuiAction::Target(*target.id(), TargetAction::Set(new_target))];
                    }
                }
            },
        };

        vec![]
    }
}
