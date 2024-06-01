use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_event::<SwitchGuiDisplay>()
            .add_systems(Update, switch_gui_display);
    }
}

#[derive(Resource, Default)]
pub struct GameState {
    gui: bool,
}

impl GameState {
    pub fn gui(&self) -> bool {
        self.gui
    }
}

#[derive(Event)]
pub struct SwitchGuiDisplay;

fn switch_gui_display(
    mut state: ResMut<GameState>,
    mut switch_gui_display: EventReader<SwitchGuiDisplay>,
) {
    if switch_gui_display
        .iter()
        .collect::<Vec<&SwitchGuiDisplay>>()
        .last()
        .is_some()
    {
        state.gui = !state.gui
    }
}
