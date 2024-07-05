use bevy_egui::egui::{ComboBox, Grid, Ui, Vec2};

use super::{paint::Painter, GuiAction};

impl<'a> Painter<'a> {
    pub fn stocks(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        let storages = self.game().materials().storages();
        let mut selected_storage = self.state().storage();
        let selected_text = self
            .state()
            .storage()
            .map(|p| format!("{}x{}", p.0 .0, p.1 .0))
            .unwrap_or("".to_string());
        ComboBox::from_label("Storages")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for (point, _) in storages {
                    ui.selectable_value(
                        &mut selected_storage,
                        Some(*point),
                        &format!("{}x{}", point.0 .0, point.1 .0),
                    );
                }
            });
        self.state_mut().set_storage(selected_storage);

        if let Some(point) = self.state().storage() {
            if let Some((_, stocks)) = self
                .game()
                .materials()
                .storages()
                .iter()
                .find(|(p, _)| p == &point)
            {
                Grid::new("stock")
                    .min_col_width(175.)
                    .min_row_height(50.)
                    .spacing(Vec2::new(10., 10.))
                    .striped(true)
                    .show(ui, |ui| {
                        for (material, quantity) in stocks {
                            ui.label(material.to_string());
                            self.illustration(ui, material);
                            ui.label(material.quantity_string(quantity));
                            ui.end_row();
                        }
                    });
            }
        }

        vec![]
    }
}
