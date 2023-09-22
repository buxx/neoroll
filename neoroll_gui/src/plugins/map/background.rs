use bevy::{prelude::*, window::WindowResized};

#[derive(Component)]
pub struct Background;

#[derive(Event)]
pub struct MapBackgroundNeedResize(pub WindowResized);
