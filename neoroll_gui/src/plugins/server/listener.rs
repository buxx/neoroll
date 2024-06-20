use bevy::prelude::*;

use neoroll_server::{
    server::{ClientMessage, ServerMessage},
    state::game::ServerGameMessage,
    subscriptions::SubscriptionsMessage,
};
use neoroll_world::{
    entity::creature::{CreatureId, PartialCreatureChange},
    space::part::{
        WorldPartCreatureMessage, WorldPartFloorMessage, WorldPartMaterialMessage,
        WorldPartMessage, WorldPartStructureMessage,
    },
};

use crate::{
    plugins::{
        game::GameStateWrapper,
        map::container::{MapPartContainer, MapPartContainerRefreshed},
        world::{
            container::{WorldPartContainer, WorldPartContainerRefreshed},
            creature::{
                display_progress, CreatureComponent, CreaturesMap, ProgressDone, ProgressMap,
                ProgressTotal,
            },
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
    mut progress_map: ResMut<ProgressMap>,
    mut commands: Commands,
    mut world_container_refreshed: EventWriter<WorldPartContainerRefreshed>,
    mut world_part: ResMut<WorldPartContainer>,
    mut map_container_refreshed: EventWriter<MapPartContainerRefreshed>,
    mut map_part: ResMut<MapPartContainer>,
    mut creatures: Query<(&CreatureComponent, &mut Transform)>,
    mut progress_done: Query<
        &mut Transform,
        (
            With<ProgressDone>,
            Without<ProgressTotal>,
            Without<CreatureComponent>,
        ),
    >,
    mut game_state: ResMut<GameStateWrapper>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                            }
                        }
                        PartialCreatureChange::SetJob(job) => {
                            creature.set_job(job);
                        }
                        // FIXME BS NOW: need big refactor here
                        PartialCreatureChange::SetBehavior(behavior) => {
                            // Update our world state part creature
                            creature.set_behavior(behavior.clone());
                            // Update bevy component display
                            display_progress(
                                creature,
                                behavior.progress(),
                                &mut progress_map,
                                &mut commands,
                                &mut progress_done,
                                &mut meshes,
                                &mut materials,
                            );
                        }
                    }
                }
            }
            ServerMessage::NewClientGameState(state) => game_state.set_state(Some(state)),
            ServerMessage::Game(message) => match message {
                ServerGameMessage::TryBuildError(original, error) => {
                    info!("TODO: (in gui) build error ({:?}): {:?}", original, error)
                }
            },
            ServerMessage::WorldPart(change) => match change {
                WorldPartMessage::Structure(point, change) => match change {
                    WorldPartStructureMessage::Set(structure) => {
                        world_part.0.set_structure(&point, structure);
                        world_container_refreshed.send(WorldPartContainerRefreshed);
                    }
                },
                WorldPartMessage::Creature(_, change) => match change {
                    WorldPartCreatureMessage::New(creature) => {
                        gateway.send(ClientMessage::Subscriptions(
                            SubscriptionsMessage::PushCreatures(*creature.id()),
                        ));
                        world_part.0.add_creature(creature);
                        world_container_refreshed.send(WorldPartContainerRefreshed);
                    }
                },
                WorldPartMessage::Floor(point, change) => match change {
                    WorldPartFloorMessage::Set(floor) => {
                        world_part.0.set_floor(&point, floor);
                        world_container_refreshed.send(WorldPartContainerRefreshed);
                    }
                },
                WorldPartMessage::Material(point, change) => match change {
                    WorldPartMaterialMessage::Set(materials) => {
                        world_part.0.set_materials(&point, materials)
                    }
                },
            },
        }
    }
}
