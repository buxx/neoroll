use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputState {
    cursor: Vec2,
    clicking: Option<(MouseButton, Vec2)>,
    clicked: Option<(MouseButton, Vec2)>,
}

impl InputState {
    pub fn cursor(&self) -> Vec2 {
        self.cursor
    }

    pub fn clicking(&self) -> Option<(MouseButton, Vec2)> {
        self.clicking
    }

    pub fn start_clicking(&mut self, button: MouseButton, point: Vec2) {
        self.clicking = Some((button, point))
    }

    pub fn end_clicking(&mut self, button: MouseButton, point: Vec2) {
        if let Some((button_, point_)) = self.clicking() {
            if button_ == button && point_ == point {
                self.clicked = Some((button, point));
            }
        }
        self.clicking = None;
    }

    pub fn reset_clicked(&mut self) {
        self.clicked = None;
    }

    pub fn cursor_mut(&mut self) -> &mut Vec2 {
        &mut self.cursor
    }

    pub fn clicked(&self) -> Option<(MouseButton, Vec2)> {
        self.clicked
    }
}
