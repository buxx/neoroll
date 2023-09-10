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
    if tileset.handle.is_none() || *has_ran || !tilesets.contains_name("Regions") {
        return;
    }

    let handle = tileset.handle.as_ref().unwrap();
    if tilesets.get(handle).is_some() {
        println!("Got tileset by handle! ({:?})", tileset.handle);
    }
    if let Some(tileset) = tilesets.get_by_id(&0) {
        println!("Got tileset by ID! ({})", tileset.id());
    }
    if let Some(tileset) = tilesets.get_by_name("Regions") {
        println!("Got tileset by name! ({})", tileset.name());
        println!("{:#?}", tileset);

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
