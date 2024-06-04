use std::collections::HashMap;

use bevy::{prelude::*, render::view::RenderLayers, sprite::MaterialMesh2dBundle};
use bevy_tileset::prelude::TileIndex;
use neoroll_world::{
    entity::creature::{CreatureId, PartialCreature},
    gameplay::progress::Progress,
};

use crate::{
    graphics::{REGION_TILE_HEIGHT, REGION_TILE_WIDTH},
    layer::LAYER_SCENE_ITEMS,
    scene::ScenePoint,
    shortcut::progress::progress_bundle,
};

pub const PROGRESS_TOTAL_Z: f32 = 2.;
pub const PROGRESS_DONE_Z: f32 = 3.;

#[derive(Component)]
pub struct CreatureComponent(pub CreatureId);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CreaturesMap(pub HashMap<CreatureId, Entity>);

#[derive(Component)]
pub struct ProgressTotal;
#[derive(Component)]
pub struct ProgressDone;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ProgressMap(pub HashMap<CreatureId, (Entity, Entity)>);

pub fn spawn_creature(
    id: CreatureId,
    atlas: &Handle<TextureAtlas>,
    tile_index: &TileIndex,
    point: Vec3,
    color: Color,
) -> (CreatureComponent, SpriteSheetBundle, RenderLayers) {
    (
        CreatureComponent(id),
        match tile_index {
            TileIndex::Standard(index) => {
                let mut sprite = TextureAtlasSprite::new(*index);
                sprite.color = color;
                SpriteSheetBundle {
                    transform: Transform {
                        translation: point,
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
    )
}

pub fn spawn_progress(
    creature: &PartialCreature,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
    progress_map: &mut ResMut<ProgressMap>,
) {
    let point = ScenePoint::from_world_point(creature.point());
    let height_offset = REGION_TILE_HEIGHT as f32 / 2.;
    let point = point.apply(0., -height_offset);
    let (total, done) = (
        (
            ProgressTotal,
            progress_bundle(point, meshes, materials, Color::BLACK, 1.),
        ),
        (
            ProgressDone,
            progress_bundle(point, meshes, materials, Color::GREEN, 0.),
        ),
    );

    let (total_entity, done_entity) = (commands.spawn(total).id(), commands.spawn(done).id());
    progress_map.insert(*creature.id(), (total_entity, done_entity));
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn display_progress(
    creature: &mut PartialCreature,
    progress: Option<&Progress>,
    progress_map: &mut ResMut<ProgressMap>,
    commands: &mut Commands,
    progress_done: &mut Query<
        &mut Transform,
        (
            With<ProgressDone>,
            Without<ProgressTotal>,
            Without<CreatureComponent>,
        ),
    >,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    if let Some(progress) = progress {
        let height_offset = REGION_TILE_HEIGHT as f32 / 2.;
        let point = ScenePoint::from_world_point(creature.point()).apply(0., -height_offset);

        // If not in progress_map, spawn
        if progress_map.get(creature.id()).is_none() {
            spawn_progress(creature, meshes, materials, commands, progress_map);
        }

        if let Some((_, done_entity)) = &progress_map.get(creature.id()) {
            if let Ok(mut done_transform) = progress_done.get_mut(*done_entity) {
                let progress_factor = (*progress).into();
                let missing_width: f32 =
                    (REGION_TILE_WIDTH as f32 - (REGION_TILE_WIDTH as f32 * progress_factor)) / 2.;
                // let width_offset: f32 = REGION_TILE_WIDTH as f32 / 2.;
                // let width_offset: f32 = width_offset - (REGION_TILE_WIDTH as f32 * progress_factor);
                let point = point.apply(-missing_width, 0.);
                done_transform.translation = point.to_vec3(PROGRESS_DONE_Z);
                done_transform.scale = Vec3::new(progress_factor, 1., 1.);
            }
        } else {
            error!(
                "Progress total/done '{}' not found when dispatching `SetBehavior`",
                creature.id()
            )
        }
    } else {
        despawn_progress(creature.id(), commands, progress_map);
    }
}

pub fn despawn_progress(
    creature_id: &CreatureId,
    commands: &mut Commands,
    progress_map: &mut ResMut<ProgressMap>,
) {
    if let Some((total_entity, done_entity)) = progress_map.get(creature_id) {
        commands.entity(*total_entity).despawn();
        commands.entity(*done_entity).despawn();
        progress_map.remove(creature_id);
    } else {
        error!(
            "Progress total/done '{}' not found when dispatching `SetBehavior`",
            creature_id
        )
    }
}
