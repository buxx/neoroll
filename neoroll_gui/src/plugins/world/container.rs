use bevy::prelude::*;
use neoroll_server::subscriptions::SubscriptionsMessage;
use neoroll_world::space::part::WorldPart;

use crate::{
    camera::{camera_world_area, SceneItemsCamera},
    plugins::server::gateway::GatewayWrapper,
    server::ClientMessage,
};

// use super::updater::WorldUpdater;

// TODO: rename (when used when server new layers received)
#[derive(Event)]
pub struct WorldPartContainerRefreshed;

// TODO: rename (when used when server new sectors received)
#[derive(Event)]
pub struct WorldPartContainerNeedRefresh;

#[derive(Resource)]
pub struct WorldPartContainer(pub WorldPart);

pub const OVER_SCREEN_TILES: isize = 30;

impl Default for WorldPartContainer {
    fn default() -> Self {
        Self(WorldPart::empty())
    }
}

impl WorldPartContainer {
    pub fn world_part(&self) -> &WorldPart {
        &self.0
    }
}

pub fn refresh_world_part_container(
    camera: Query<(&SceneItemsCamera, &Camera, &mut Transform)>,
    gateway: Res<GatewayWrapper>,
    // world_updater: ResMut<WorldUpdater>,
    world_part: ResMut<WorldPartContainer>,
    mut world_container_need_refresh: EventReader<WorldPartContainerNeedRefresh>,
    // mut world_container_refreshed: EventWriter<WorldPartContainerRefreshed>,
) {
    if !world_container_need_refresh
        .iter()
        .collect::<Vec<&WorldPartContainerNeedRefresh>>()
        .is_empty()
    {
        let (_, camera, transform) = camera.single();
        let target = camera.physical_target_size().unwrap_or(UVec2::new(0, 0));
        let translation = transform.translation;
        let scale = transform.scale;
        let area = camera_world_area(target, translation, scale)
            .resize(OVER_SCREEN_TILES, OVER_SCREEN_TILES);

        // FIXME BS NOW : fix a max area size to retrieve only needed area when on
        // map (to be able to compute path finding without download all world ...)
        // For now, when very little zoom, a lot of tile are loaded !!
        let current_area = world_part.0.area();
        gateway.send(ClientMessage::Subscriptions(SubscriptionsMessage::SetArea(
            area.clone(),
        )));
        gateway.send(ClientMessage::RequireWorldArea(area, current_area.clone()));

        // world_updater.update(&server_gateway, &mut world_container, area);
        // world_container_refreshed.send(WorldPartContainerRefreshed);
    }
}
