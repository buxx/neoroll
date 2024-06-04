use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    graphics::{REGION_TILE_HEIGHT, REGION_TILE_WIDTH},
    plugins::world::creature::PROGRESS_TOTAL_Z,
    scene::ScenePoint,
};

pub fn progress_bundle(
    point: ScenePoint,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    progress: f32,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(
                REGION_TILE_WIDTH as f32,
                REGION_TILE_HEIGHT as f32 / 8.,
            ))))
            .into(),
        transform: Transform {
            translation: point.to_vec3(PROGRESS_TOTAL_Z),
            scale: Vec3::new(progress, 1., 1.),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(color)),
        ..default()
    }
}
