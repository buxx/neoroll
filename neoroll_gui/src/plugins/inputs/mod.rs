use bevy::prelude::*;

use self::{
    drag::{on_dragged_screen, DraggedScreen},
    state::InputState,
    update::update_inputs,
    window::on_window_resize,
};

pub mod drag;
pub mod state;
pub mod update;
pub mod window;

pub struct UserInputsPlugin;

impl Plugin for UserInputsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_event::<DraggedScreen>()
            .add_systems(Update, (update_inputs, on_dragged_screen, on_window_resize));
    }
}
