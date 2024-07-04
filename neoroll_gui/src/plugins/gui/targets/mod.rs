pub mod keep_stock;

use bevy_egui::egui::{ComboBox, Grid, Ui, Vec2};
use neoroll_server::state::game::settings::TargetSetting;
use neoroll_world::gameplay::material::Material;
use neoroll_world::gameplay::target::{ComputedTarget, Target, TargetId};

use crate::plugins::gui::state::{AddKeepStockTargetMaterial, AddTarget};
use crate::plugins::gui::TargetAction;

use super::GuiAction;

use super::paint::Painter;

impl<'a> Painter<'a> {
    pub fn targets(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        let mut actions = vec![];

        actions.extend(self.add_target(ui));
        ui.separator();
        actions.extend(self.active_targets(ui));

        actions
    }

    fn active_targets(&self, ui: &mut Ui) -> Vec<GuiAction> {
        let mut actions = vec![];

        Grid::new("targets")
            .min_col_width(115.)
            .spacing(Vec2::new(10., 10.))
            .striped(true)
            .show(ui, |ui| {
                let mut targets: Vec<&ComputedTarget> =
                    self.game().target().targets().values().collect();
                targets.sort_by_key(|t| t.priority());

                for target in targets {
                    actions.extend(self.target_row(ui, target));
                    ui.end_row();
                }
            });

        actions
    }

    fn target_row(&self, ui: &mut Ui, target: &ComputedTarget) -> Vec<GuiAction> {
        let mut actions = vec![];
        let target_ = target.target();

        ui.label(format!("{}", target.priority()));

        ui.horizontal(|ui| {
            ui.label(target_.name());
        });

        match target_ {
            Target::KeepStock(_, _) => {
                actions.extend(self.keep_stock_resume(ui, target));
                if let Some(material) = target_.material() {
                    self.illustration(ui, material);
                } else {
                    ui.label("");
                }
                ui.label("progress TODO");
                actions.extend(self.keep_stock_settings(ui, target));
            }
        }

        actions
    }

    // TODO: Make this more dynamic
    fn add_target(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        let mut actions = vec![];

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let add_target = self.state_mut().add_target_selection_mut();
                ui.label("Add target");
                ComboBox::from_id_source("add_target")
                    .selected_text(add_target.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            add_target,
                            AddTarget::None,
                            &AddTarget::None.to_string(),
                        );
                        ui.selectable_value(
                            add_target,
                            AddTarget::KeepStock,
                            &AddTarget::KeepStock.to_string(),
                        );
                    });

                match add_target {
                    AddTarget::None => {}
                    AddTarget::KeepStock => {
                        let material = self.state_mut().add_keep_stock_material_mut();

                        ComboBox::from_id_source("add_keep_stock_material")
                            .selected_text(material.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    material,
                                    AddKeepStockTargetMaterial::Food,
                                    &AddKeepStockTargetMaterial::Food.to_string(),
                                );
                                ui.selectable_value(
                                    material,
                                    AddKeepStockTargetMaterial::RawFlint,
                                    &AddKeepStockTargetMaterial::RawFlint.to_string(),
                                );
                            });

                        if ui.button("Add").clicked() {
                            let material: Material = material.to_owned().into();
                            let priority = self.game().target().targets().len() + 1;
                            actions.extend(vec![GuiAction::Target(
                                TargetId::new(),
                                TargetAction::New(TargetSetting::new(
                                    Target::KeepStock(material, Default::default()).default(),
                                    priority,
                                )),
                            )]);
                        }
                    }
                };
            });
        });

        actions
    }
}
