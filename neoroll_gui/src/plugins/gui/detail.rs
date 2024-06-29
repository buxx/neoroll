use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::{EventReader, Query, ResMut, With, Without},
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
};
use bevy_egui::egui::Ui;
use neoroll_world::{entity::creature::PartialCreature, space::AbsoluteWorldPoint};

use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    plugins::{inputs::state::InputState, world::container::WorldPartContainer},
    scene::{FromScenePoint, ScenePoint},
};

use super::{paint::Painter, state::GuiState, Current, GuiAction, Panel};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn details(
    mut state: ResMut<GuiState>,
    input_state: ResMut<InputState>,
    mut mouse: EventReader<MouseButtonInput>,
    camera: Query<(&Camera, &GlobalTransform), (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
    world_part: ResMut<WorldPartContainer>,
    windows: Query<&Window>,
) {
    // TODO: refactor this big code part (click => point)
    if let Some(event) = mouse.iter().last() {
        if let ButtonState::Released = event.state {
            let point = input_state.cursor();
            if let Some(click) = input_state.click() {
                if click.1 == point {
                    if let Current::Explore = state.current_mode() {
                        let window = windows.single();
                        let (camera, camera_transform) = camera.single();
                        if let Some(world_position) = window.cursor_position().and_then(|cursor| {
                            camera.viewport_to_world_2d(camera_transform, cursor)
                        }) {
                            let point = AbsoluteWorldPoint::from_scene_point(ScenePoint::new(
                                world_position.x,
                                -world_position.y,
                            ));

                            if let Some((id, _)) = world_part
                                .0
                                .creatures()
                                .iter()
                                .find(|(_, c)| c.point() == &point)
                            {
                                println!("1");
                                if state.selected().creature().is_none() {
                                    println!("2");
                                    state.selected_mut().select_creature(*id);
                                } else {
                                    println!("3");
                                    state.selected_mut().select_tile(point);
                                }
                            } else {
                                println!("4");
                                state.selected_mut().select_tile(point);
                            }

                            *state.display_window_mut() = true;
                            *state.current_panel_mut() = Panel::Details;
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Painter<'a> {
    pub fn details(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        let mut actions = vec![];

        let selected = self.state().selected();
        if let Some(creature_id) = selected.creature() {
            if let Some(creature) = self.world().creature(&creature_id) {
                actions.extend(self.creature_detail(ui, creature));
            }
        } else if let Some(point) = selected.tile() {
            ui.label(format!("{:?}", point));
        } else {
            ui.label("nothing");
        }

        actions
    }

    fn creature_detail(&self, ui: &mut Ui, creature: &PartialCreature) -> Vec<GuiAction> {
        ui.label("Creature");
        ui.label(format!("Job: {}", creature.job()));
        ui.label(format!("Behavior: {}", creature.behavior()));
        vec![]
    }
}
