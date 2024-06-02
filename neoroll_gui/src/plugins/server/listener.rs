use bevy::prelude::*;

use neoroll_server::{
    server::{ClientMessage, ServerMessage},
    subscriptions::SubscriptionsMessage,
};
use neoroll_world::entity::creature::{CreatureId, PartialCreatureChange};

use crate::{
    plugins::{
        game::GameStateWrapper,
        map::container::{MapPartContainer, MapPartContainerRefreshed},
        world::{
            container::{WorldPartContainer, WorldPartContainerRefreshed},
            creature::{CreatureComponent, CreaturesMap},
        },
    },
    scene::ScenePoint,
};

use super::gateway::GatewayWrapper;

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn listen(
    gateway: Res<GatewayWrapper>,
    creatures_map: Res<CreaturesMap>,
    mut world_container_refreshed: EventWriter<WorldPartContainerRefreshed>,
    mut world_part: ResMut<WorldPartContainer>,
    mut map_container_refreshed: EventWriter<MapPartContainerRefreshed>,
    mut map_part: ResMut<MapPartContainer>,
    mut creatures: Query<(&CreatureComponent, &mut Transform)>,
    mut game_state: ResMut<GameStateWrapper>,
) {
    for message in gateway.read() {
        // TODO: dispatch code in modules/plugins
        match message {
            ServerMessage::Hello(_) => {
                // TODO: heartbeat
            }
            ServerMessage::NewWorldLayers(area, layers) => {
                debug!("Received {} tiles", layers.len());

                // Subscribe on each received creatures
                let creature_ids: Vec<CreatureId> =
                    layers.creatures().iter().map(|c| *c.id()).collect();
                gateway.send(ClientMessage::Subscriptions(
                    SubscriptionsMessage::SetCreatures(creature_ids),
                ));

                // Update gui world part
                world_part.0.switch(layers, area);
                world_container_refreshed.send(WorldPartContainerRefreshed);
            }
            ServerMessage::NewMapSectors(area, sectors) => {
                debug!("Received {} sectors", sectors.len());
                // TODO: hardcoded lakes for now
                map_part.0.switch(sectors, vec![].clone(), area);
                map_container_refreshed.send(MapPartContainerRefreshed);
            }
            ServerMessage::Creature(id, change) => {
                if let Some(creature) = world_part.0.creature_mut(&id) {
                    match change {
                        PartialCreatureChange::SetPoint(point) => {
                            // Update our world state part creature
                            creature.set_point(point);
                            // Update bevy component display
                            let point: Vec3 = ScenePoint::from_world_point(creature.point()).into();
                            if let Some(entity) = creatures_map.get(&id) {
                                if let Ok((_, mut transform)) = creatures.get_mut(*entity) {
                                    transform.translation = point;
                                }
                            } else {
                                error!("Creature '{}' not found when dispatching `SetPoint`", &id)
                            }
                        }
                    }
                }
            }
            ServerMessage::NewClientGameState(state) => game_state.set_state(Some(state)),
        }
    }
}
