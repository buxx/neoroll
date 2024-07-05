use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    render::view::RenderLayers,
    sprite::MaterialMesh2dBundle,
};
use bevy_egui::egui::{self, Ui};
use bevy_tileset::prelude::{TileIndex, Tilesets};
use neoroll_server::{server::ClientMessage, state::game::ClientGameMessage};
use neoroll_world::{gameplay::build::Buildable, space::AbsoluteWorldPoint};

use crate::{
    camera::{BackgroundCamera, SceneItemsCamera},
    graphics::{REGION_TILE_HEIGHT, REGION_TILE_WIDTH},
    image::Illustration,
    layer::LAYER_SCENE_ITEMS,
    plugins::{server::gateway::GatewayWrapper, world::tileset::WORLD_TILESET_NAME},
    scene::{FromScenePoint, ScenePoint},
    utils::TileName,
};

use super::{paint::Painter, Current, GuiAction, GuiState};

#[derive(Component)]
pub struct BuildCursor;

#[derive(Component)]
pub struct BuildOutline;

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn display_build_cursor(
    state: Res<GuiState>,
    windows: Query<&Window>,
    mut cursor: Query<(&BuildCursor, &mut Transform)>,
    camera: Query<(&Camera, &GlobalTransform), (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
) {
    if let Current::Build(_) = state.current_mode() {
        let window = windows.single();
        if let Ok((_, mut transform)) = cursor.get_single_mut() {
            let (camera, camera_transform) = camera.single();
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                transform.translation = Vec3::new(
                    world_position.x - (REGION_TILE_WIDTH / 2) as f32,
                    world_position.y + (REGION_TILE_HEIGHT / 2) as f32,
                    2., // Note here: to be front of outline
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn display_build_outline(
    state: Res<GuiState>,
    windows: Query<&Window>,
    mut outline: Query<(&BuildOutline, &mut Transform)>,
    camera: Query<(&Camera, &GlobalTransform), (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
) {
    if let Current::Build(_) = state.current_mode() {
        let window = windows.single();
        if let Ok((_, mut transform)) = outline.get_single_mut() {
            let (camera, camera_transform) = camera.single();
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                let tile_point = tile_point_from_world_xy(world_position);
                transform.translation = ScenePoint::from_world_point(&tile_point).to_vec3(1.);
            }
        }
    }
}

pub fn spawn_build_cursor(commands: &mut Commands, buildable: Buildable, tilesets: &Tilesets) {
    if let Some(tileset) = tilesets.get_by_name(WORLD_TILESET_NAME) {
        let atlas = tileset.atlas();
        let (tile_index, _) = &tileset.select_tile(buildable.tile_name()).unwrap();
        commands.spawn((
            BuildCursor,
            match tile_index {
                // TODO: refactor with other match like this
                TileIndex::Standard(index) => {
                    let sprite = TextureAtlasSprite::new(*index);
                    SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(0., 0., 0.),
                            ..Default::default()
                        },
                        sprite,
                        texture_atlas: atlas.clone(),
                        ..Default::default()
                    }
                }
                TileIndex::Animated(_start, _end, _speed) => {
                    todo!()
                }
            },
            RenderLayers::layer(LAYER_SCENE_ITEMS),
        ));
    }
}

pub fn spawn_build_outline(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    commands.spawn((
        BuildOutline,
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(
                    REGION_TILE_WIDTH as f32,
                    REGION_TILE_HEIGHT as f32,
                ))))
                .into(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            ..default()
        },
    ));
}

pub fn despawn_build_cursor(commands: &mut Commands, cursor: Query<Entity, With<BuildCursor>>) {
    commands.entity(cursor.single()).despawn();
}

pub fn despawn_build_outline(commands: &mut Commands, cursor: Query<Entity, With<BuildOutline>>) {
    commands.entity(cursor.single()).despawn();
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn try_build(
    mut state: ResMut<GuiState>,
    gateway: Res<GatewayWrapper>,
    windows: Query<&Window>,
    cursor: Query<Entity, With<BuildCursor>>,
    outline: Query<Entity, With<BuildOutline>>,
    camera: Query<(&Camera, &GlobalTransform), (With<SceneItemsCamera>, Without<BackgroundCamera>)>,
    mut mouse: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    if let Current::Build(building) = state.current_mode() {
        if let Some(event) = mouse.iter().last() {
            if let ButtonState::Pressed = event.state {
                let window = windows.single();
                let (camera, camera_transform) = camera.single();
                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                {
                    let point = tile_point_from_world_xy(world_position);
                    gateway.send(ClientMessage::Game(ClientGameMessage::TryBuild(
                        *building, point,
                    )));
                    state.set_current(Current::Explore);

                    despawn_build_cursor(&mut commands, cursor);
                    despawn_build_outline(&mut commands, outline);
                }
            }
        }
    }
}

fn tile_point_from_world_xy(position: Vec2) -> AbsoluteWorldPoint {
    AbsoluteWorldPoint::from_scene_point(ScenePoint::new(position.x, -position.y))
}

impl<'a> Painter<'a> {
    pub fn builds(&mut self, ui: &mut Ui) -> Vec<GuiAction> {
        let mut actions = vec![];

        ui.horizontal_wrapped(|ui| {
            if self.game().build().can_build_campfire()
                && ui
                    .add_sized([75., 75.], egui::ImageButton::new(Illustration::CampfireButton.data()))
                    .clicked()
            {
                actions.extend(vec![GuiAction::Build(Buildable::Campfire)]);
            };

            if self.game().build().can_build_storage()
                && ui
                    .add_sized([75., 75.], egui::ImageButton::new(Illustration::ShortAndDryGrassButton.data()))
                    .clicked()
            {
                actions.extend(vec![GuiAction::Build(Buildable::Storage)]);
            };
        });

        actions
    }
}
