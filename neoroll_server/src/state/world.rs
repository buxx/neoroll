use std::sync::{Arc, RwLock};

use neoroll_world::{
    entity::creature::{CreatureChange, CreatureId, PartialCreatureChange},
    space::{
        part::{
            WorldPartCreatureMessage, WorldPartFloorMessage, WorldPartMessage,
            WorldPartStructureMessage,
        },
        world::{FloorChange, MaterialChange, StructureChange, World, WorldChange},
        AbsoluteWorldPoint,
    },
};

use crate::{
    gateway::Gateways,
    server::{ServerMessage, ServerMessageEnveloppe},
    subscriptions::Subscriptions,
};

use super::game::GameState;

pub struct WorldModifier<'a> {
    gateways: &'a Arc<RwLock<Gateways>>,
    subscriptions: &'a Arc<RwLock<Subscriptions>>,
    world: &'a mut World,
    game: &'a mut GameState,
}

impl<'a> WorldModifier<'a> {
    pub fn new(
        gateways: &'a Arc<RwLock<Gateways>>,
        subscriptions: &'a Arc<RwLock<Subscriptions>>,
        world: &'a mut World,
        game: &'a mut GameState,
    ) -> Self {
        Self {
            gateways,
            subscriptions,
            world,
            game,
        }
    }

    fn send_to_point_clients(&self, point: &AbsoluteWorldPoint, message: ServerMessage) {
        for client_id in self.subscriptions.read().unwrap().to_point(point) {
            self.gateways
                .read()
                .unwrap()
                .send(ServerMessageEnveloppe::To(client_id, message.clone()))
                .unwrap();
        }
    }

    fn send_to_creature_clients(&self, id: &CreatureId, message: ServerMessage) {
        for client_id in self.subscriptions.read().unwrap().to_creature(id) {
            self.gateways
                .read()
                .unwrap()
                .send(ServerMessageEnveloppe::To(client_id, message.clone()))
                .unwrap();
        }
    }

    pub fn apply(&mut self, change: WorldChange) {
        match change {
            WorldChange::Creature(id, change) => match change {
                CreatureChange::New(creature) => {
                    let point = *creature.point();
                    self.world.add_creature(creature.clone());
                    self.send_to_point_clients(
                        &point,
                        ServerMessage::WorldPart(WorldPartMessage::Creature(
                            *creature.id(),
                            WorldPartCreatureMessage::New(creature.clone().into()),
                        )),
                    );
                }
                CreatureChange::SetPoint(point) => {
                    self.world
                        .creatures_mut()
                        .get_mut(&id)
                        .unwrap()
                        .set_point(point);
                    self.send_to_creature_clients(
                        &id,
                        ServerMessage::Creature(id, PartialCreatureChange::SetPoint(point)),
                    );
                }
                CreatureChange::SetJob(job) => {
                    self.world
                        .creatures_mut()
                        .get_mut(&id)
                        .unwrap()
                        .set_job(job.clone());
                    self.send_to_creature_clients(
                        &id,
                        ServerMessage::Creature(id, PartialCreatureChange::SetJob(job)),
                    );
                }
                CreatureChange::SetBehavior(behavior) => {
                    self.world
                        .creatures_mut()
                        .get_mut(&id)
                        .unwrap()
                        .set_behavior(behavior.clone());
                    self.send_to_creature_clients(
                        &id,
                        ServerMessage::Creature(id, PartialCreatureChange::SetBehavior(behavior)),
                    );
                }
                CreatureChange::AddToCarrying(material, quantity) => {
                    self.world
                        .creatures_mut()
                        .get_mut(&id)
                        .unwrap()
                        .add_to_carrying(material, quantity);
                }
                CreatureChange::RemoveFromCarrying(material, quantity) => {
                    self.world
                        .creatures_mut()
                        .get_mut(&id)
                        .unwrap()
                        .remove_from_carrying(material, quantity);
                }
            },
            WorldChange::Structure(point, change) => match change {
                StructureChange::Set(structure) => {
                    // FIXME: if None, destroy game.structures_own related objects
                    self.world.set_structure(point, structure.clone());

                    self.send_to_point_clients(
                        &point,
                        ServerMessage::WorldPart(WorldPartMessage::Structure(
                            point,
                            WorldPartStructureMessage::Set(structure.clone()),
                        )),
                    );
                }
                StructureChange::SetOwned(own) => {
                    self.world
                        .set_structure(*own.point(), Some(own.type_().clone()));
                    self.game.set_structure_own(own.clone());

                    self.send_to_point_clients(
                        &point,
                        ServerMessage::WorldPart(WorldPartMessage::Structure(
                            point,
                            WorldPartStructureMessage::Set(Some(own.type_().clone())),
                        )),
                    );
                }
            },
            WorldChange::Floor(point, change) => match change {
                FloorChange::Set(floor) => {
                    self.world.set_floor(point, floor.clone());

                    self.send_to_point_clients(
                        &point,
                        ServerMessage::WorldPart(WorldPartMessage::Floor(
                            point,
                            WorldPartFloorMessage::Set(floor.clone()),
                        )),
                    );
                }
            },
            WorldChange::Material(point, change) => match change {
                MaterialChange::Add(material, quantity) => {
                    self.world.add_material(point, material, quantity);
                }
            },
        }
    }
}
