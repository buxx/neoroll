use bevy::prelude::*;
use bevy_tileset::prelude::*;

#[derive(Resource, Default)]
pub struct RegionTileset {
    pub handle: Option<Handle<Tileset>>,
}

pub fn show_tileset(
    tilesets: Tilesets,
    mut commands: Commands,
    tileset: Res<RegionTileset>,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    if let (Some(_), Some(tileset)) = (&tileset.handle, tilesets.get_by_name("Regions")) {
        let texture = tileset.texture().clone();
        commands.spawn(Camera2dBundle::default());
        commands.spawn(SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });

        *has_ran = true;
    }
}
