use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputState {
    cursor: Vec2,
    click: Option<(MouseButton, Vec2)>,
}

impl InputState {
    pub fn cursor(&self) -> Vec2 {
        self.cursor
    }

    pub fn click(&self) -> Option<(MouseButton, Vec2)> {
        self.click
    }

    pub fn start_click(&mut self, button: MouseButton, point: Vec2) {
        self.click = Some((button, point))
    }

    pub fn end_click(&mut self) {
        self.click = None;
    }

    pub fn cursor_mut(&mut self) -> &mut Vec2 {
        &mut self.cursor
    }
}
