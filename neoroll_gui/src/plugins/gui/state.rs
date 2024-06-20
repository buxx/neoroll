use bevy::prelude::*;

use super::{Current, Panel};

#[derive(Resource, Default)]
pub struct GuiState {
    current_mode: Current,
    current_panel: Panel,
    display_window: bool,
    server_speed_request: u8,
    is_pointer_over_area: bool,
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
}
