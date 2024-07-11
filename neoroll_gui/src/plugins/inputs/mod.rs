use bevy::prelude::*;
use update::{update_clicks, update_keyboard, update_motion, update_wheel};

use self::{
    drag::{on_dragged_screen, DraggedScreen},
    state::InputState,
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
            .add_systems(
                Update,
                (
                    update_keyboard,
                    update_wheel,
                    update_motion,
                    update_clicks,
                    on_dragged_screen,
                    on_window_resize,
                ),
            );
    }
}
