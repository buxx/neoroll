pub mod targets;

pub mod paint;
pub mod root;

use std::fmt::Display;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use bevy_tileset::prelude::Tilesets;
use build::{
    display_build_cursor, display_build_outline, spawn_build_cursor, spawn_build_outline, try_build,
};
use neoroll_server::{
    server::ClientMessage,
    state::game::{ClientGameMessage, TargetMessage},
};
use neoroll_world::gameplay::{
    build::Buildable,
    target::{Target, TargetId},
};
use paint::Painter;
use state::GuiState;
use strum_macros::EnumIter;

use crate::utils::{EventReaderShortcuts, TileName};

use super::{game::GameStateWrapper, server::gateway::GatewayWrapper};

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
                    switch_gui_display,
                    display_build_cursor,
                    display_build_outline,
                    try_build,
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
pub struct SwitchDisplayWindow;

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
) {
    let mut hover = false;
    let mut effects = vec![];

    if state.display_window() {
        let ctx = contexts.ctx_mut();

        egui::Window::new("").show(ctx, |ui| {
            if let Some(game) = game_state.state() {
                effects.extend(Painter::new(game, &mut state, &gateway).paint(ui));
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
        }
    }
}

pub enum GuiAction {
    Build(Buildable),
    Target(TargetId, TargetAction),
}

pub enum TargetAction {
    Set(Target),
}

impl From<TargetAction> for TargetMessage {
    fn from(value: TargetAction) -> Self {
        match value {
            TargetAction::Set(target) => TargetMessage::Set(target),
        }
    }
}
