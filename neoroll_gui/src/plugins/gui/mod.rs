pub mod stock;
pub mod targets;

pub mod detail;
pub mod paint;
pub mod root;

use std::fmt::Display;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiSettings};
use bevy_tileset::prelude::Tilesets;
use build::{
    display_build_cursor, display_build_outline, spawn_build_cursor, spawn_build_outline, try_build,
};
use detail::details;
use neoroll_server::{
    server::ClientMessage,
    state::game::{settings::TargetSetting, ClientGameMessage, TargetMessage},
};
use neoroll_world::gameplay::{build::Buildable, target::TargetId};
use paint::Painter;
use state::GuiState;
use strum_macros::EnumIter;

use crate::utils::{EventReaderShortcuts, TileName};

use super::{
    game::GameStateWrapper, server::gateway::GatewayWrapper, world::container::WorldPartContainer,
};

pub mod build;
pub mod state;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiState>()
            .add_event::<SwitchDisplayWindow>()
            .add_systems(
                Update,
                (
                    gui,
                    update_ui_scale_factor,
                    switch_gui_display,
                    display_build_cursor,
                    display_build_outline,
                    try_build,
                    details,
                ),
            );
    }
}

pub enum Current {
    Explore,
    Build(Buildable),
}

impl Default for Current {
    fn default() -> Self {
        Self::Explore
    }
}

#[derive(Event)]
pub struct SwitchDisplayWindow(pub Panel);

fn switch_gui_display(
    mut state: ResMut<GuiState>,
    mut switch_gui_display: EventReader<SwitchDisplayWindow>,
) {
    if switch_gui_display.has_been_set() {
        state.switch_display_window()
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn gui(
    gateway: Res<GatewayWrapper>,
    mut state: ResMut<GuiState>,
    mut contexts: EguiContexts,
    game_state: Res<GameStateWrapper>,
    mut commands: Commands,
    tilesets: Tilesets,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_part: Res<WorldPartContainer>,
) {
    let mut hover = false;
    let mut effects = vec![];

    if state.display_window() {
        let ctx = contexts.ctx_mut();
        egui_extras::install_image_loaders(ctx);

        egui::Window::new("").auto_sized().show(ctx, |ui| {
            if let Some(game) = game_state.state() {
                effects.extend(Painter::new(game, &mut state, &world_part.0, &gateway).paint(ui));
            }
        });

        hover = ctx.is_pointer_over_area();
    }

    for effect in effects {
        match effect {
            GuiAction::Build(buildable) => {
                state.set_display_window(false);
                state.set_current(Current::Build(buildable));
                spawn_build_outline(&mut commands, &mut meshes, &mut materials);
                spawn_build_cursor(&mut commands, buildable, &tilesets);
            }
            GuiAction::Target(target_id, target_action) => gateway.send(ClientMessage::Game(
                ClientGameMessage::Target(target_id, target_action.into()),
            )),
        }
    }

    state.set_is_pointer_over_area(hover);
}

fn update_ui_scale_factor(
    mut egui_settings: ResMut<EguiSettings>,
    state: Res<GuiState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let zoom = *state.zoom();
    if let Ok(window) = windows.get_single() {
        egui_settings.scale_factor = zoom.factor() / window.scale_factor();
    }
}

impl TileName for Buildable {
    fn tile_name(&self) -> &str {
        match self {
            Buildable::Campfire => "Campfire",
            Buildable::Storage => "Storage",
        }
    }
}

#[derive(EnumIter, Eq, PartialEq)]
pub enum Panel {
    Root,
    Targets,
    Build,
    Details,
    Stock,
}

impl Default for Panel {
    fn default() -> Self {
        Self::Root
    }
}

impl Display for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Panel::Root => f.write_str("Resume"),
            Panel::Targets => f.write_str("Targets"),
            Panel::Details => f.write_str("Details"),
            Panel::Build => f.write_str("Build"),
            Panel::Stock => f.write_str("Stocks"),
        }
    }
}

pub enum GuiAction {
    Build(Buildable),
    Target(TargetId, TargetAction),
}

pub enum TargetAction {
    New(TargetSetting),
    Set(TargetSetting),
}

impl From<TargetAction> for TargetMessage {
    fn from(value: TargetAction) -> Self {
        match value {
            TargetAction::Set(target) => TargetMessage::Set(target),
            TargetAction::New(target) => TargetMessage::New(target),
        }
    }
}
