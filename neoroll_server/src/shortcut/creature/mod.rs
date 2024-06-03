use neoroll_world::{
    entity::creature::{Creature, CreatureChange, CreatureId},
    gameplay::tribe::TribeId,
    space::{world::WorldChange, AbsoluteWorldPoint},
};

use crate::{
    action::{job::realize::RealizeJobBuilder, ActionChange, ActionId},
    state::StateChange,
};

pub fn new_creature(tribe_id: TribeId, point: AbsoluteWorldPoint) -> Vec<StateChange> {
    let creature_id = CreatureId::new();
    let creature = Creature::new(creature_id, tribe_id, point);

    vec![
        StateChange::World(WorldChange::Creature(
            creature_id,
            CreatureChange::New(creature),
        )),
        StateChange::Action(
            ActionId::new(),
            ActionChange::New(RealizeJobBuilder::new(creature_id).build()),
        ),
    ]
}
