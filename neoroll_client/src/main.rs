use bevy::prelude::*;

use bevy_tileset::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TilesetPlugin::default()))
        .init_resource::<RegionTileset>()
        .add_systems(Startup, setup)
        .add_systems(Update, (show_tileset, move_camera))
        .run();
}

#[derive(Component)]
struct CameraController;

fn setup(
    mut tileset: ResMut<RegionTileset>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), CameraController));
    tileset.handle = Some(asset_server.load("tilesets/regions.ron"));
}

fn move_camera(
    mut camera: Query<&mut Transform, With<CameraController>>,
    button: Res<Input<MouseButton>>,
) {
    let mut camera = camera.single_mut();
    if button.just_pressed(MouseButton::Left) {
        println!("move camera");
        camera.translation = camera.translation.lerp(Vec3::new(1.0, 1.0, 1.0), 0.2);
    }
}

#[derive(Resource, Default)]
struct RegionTileset {
    handle: Option<Handle<Tileset>>,
}

fn show_tileset(
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
