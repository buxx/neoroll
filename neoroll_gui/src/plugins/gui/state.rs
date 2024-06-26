use std::fmt::Display;

use bevy::prelude::Resource as BevyResource;
use neoroll_world::gameplay::material::{Material, Resource};

use super::{Current, Panel};

#[derive(BevyResource, Default)]
pub struct GuiState {
    current_mode: Current,
    current_panel: Panel,
    display_window: bool,
    server_speed_request: u8,
    is_pointer_over_area: bool,

    // TODO: In separated struct
    add_target_selection: AddTarget,
    add_keep_stock_material: AddKeepStockTargetMaterial,
}

impl GuiState {
    pub fn display_window(&self) -> bool {
        self.display_window
    }

    pub fn current_mode(&self) -> &Current {
        &self.current_mode
    }

    pub fn set_current(&mut self, current: Current) {
        self.current_mode = current;
    }

    pub fn set_display_window(&mut self, display_window: bool) {
        self.display_window = display_window;
    }

    pub fn switch_display_window(&mut self) {
        self.display_window = !self.display_window;
    }

    pub fn set_is_pointer_over_area(&mut self, is_pointer_over_area: bool) {
        self.is_pointer_over_area = is_pointer_over_area;
    }

    pub fn is_pointer_over_area(&self) -> bool {
        self.is_pointer_over_area
    }

    pub fn display_window_mut(&mut self) -> &mut bool {
        &mut self.display_window
    }

    pub fn server_speed_request(&self) -> u8 {
        self.server_speed_request
    }

    pub fn server_speed_request_mut(&mut self) -> &mut u8 {
        &mut self.server_speed_request
    }

    pub fn current_panel(&self) -> &Panel {
        &self.current_panel
    }

    pub fn current_panel_mut(&mut self) -> &mut Panel {
        &mut self.current_panel
    }

    pub fn add_target_selection_mut(&mut self) -> &mut AddTarget {
        &mut self.add_target_selection
    }

    pub fn add_keep_stock_material_mut(&mut self) -> &mut AddKeepStockTargetMaterial {
        &mut self.add_keep_stock_material
    }
}

#[derive(Eq, PartialEq)]
pub enum AddTarget {
    None,
    KeepStock,
}
impl AddTarget {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl Display for AddTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddTarget::None => f.write_str(""),
            AddTarget::KeepStock => f.write_str("Keep Stock of"),
        }
    }
}

impl Default for AddTarget {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum AddKeepStockTargetMaterial {
    Food,
    RawFlint,
}

impl Display for AddKeepStockTargetMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddKeepStockTargetMaterial::Food => f.write_str("Food"),
            AddKeepStockTargetMaterial::RawFlint => f.write_str("Raw Flint"),
        }
    }
}

impl Default for AddKeepStockTargetMaterial {
    fn default() -> Self {
        Self::Food
    }
}

impl From<AddKeepStockTargetMaterial> for Material {
    fn from(value: AddKeepStockTargetMaterial) -> Self {
        match value {
            AddKeepStockTargetMaterial::Food => Material::Resource(Resource::Food),
            AddKeepStockTargetMaterial::RawFlint => Material::Resource(Resource::RawFlint),
        }
    }
}
