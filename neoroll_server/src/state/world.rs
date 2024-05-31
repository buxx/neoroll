use std::sync::{Arc, RwLock};

use neoroll_world::{
    entity::creature::{CreatureChange, PartialCreatureChange},
    space::world::{World, WorldChange},
};

use crate::{
    gateway::Gateways,
    server::{ServerMessage, ServerMessageEnveloppe},
    subscriptions::Subscriptions,
};

pub struct WorldModifier<'a> {
    gateways: &'a Arc<RwLock<Gateways>>,
    subscriptions: &'a Arc<RwLock<Subscriptions>>,
    world: &'a mut World,
}

impl<'a> WorldModifier<'a> {
    pub fn new(
        gateways: &'a Arc<RwLock<Gateways>>,
        subscriptions: &'a Arc<RwLock<Subscriptions>>,
        world: &'a mut World,
    ) -> Self {
        Self {
            gateways,
            subscriptions,
            world,
        }
    }

    pub fn apply(&mut self, change: WorldChange) {
        match change {
            WorldChange::Creature(id, change) => match change {
                CreatureChange::New(creature) => {
                    // FIXME: send this new creature to clients which are in new creature area
                    let point = *creature.point();
                    self.world.creatures_mut().insert(*creature.id(), creature);

                    for _client_id in self.subscriptions.read().unwrap().to_point(&point) {
                        // TODO: send new creature
                    }
                }
                CreatureChange::SetPoint(point) => {
                    // FIXME BS NOW: think about how to propagate to client(s)
                    // (according to subscriptions, send through gateway)
                    self.world
                        .creatures_mut()
                        .get_mut(&id)
                        .unwrap()
                        .set_point(point);

                    for client_id in self.subscriptions.read().unwrap().to_creature(&id) {
                        self.gateways
                            .read()
                            .unwrap()
                            .send(ServerMessageEnveloppe::To(
                                client_id,
                                ServerMessage::Creature(id, PartialCreatureChange::SetPoint(point)),
                            ))
                            .unwrap();
                    }
                }
            },
        }
    }
}
