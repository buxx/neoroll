use neoroll_world::{
    entity::creature::{Creature, CreatureChange, CreatureId},
    gameplay::tribe::TribeId,
    space::{world::WorldChange, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI},
};

use crate::{
    action::{move_::MoveCreatureBuilder, ActionChange, ActionId},
    state::StateChange,
};

pub fn new_creature(tribe_id: TribeId, point: AbsoluteWorldPoint) -> Vec<StateChange> {
    let creature_id = CreatureId::new();
    let creature = Creature::new(creature_id, tribe_id, point);

    // FIXME: real search food action, behavior choose, etc.
    let move_to = AbsoluteWorldPoint(AbsoluteWorldRowI(30), AbsoluteWorldColI(30));
    vec![
        StateChange::World(WorldChange::Creature(
            creature_id,
            CreatureChange::New(creature),
        )),
        StateChange::Action(
            ActionId::new(),
            ActionChange::New(MoveCreatureBuilder::new(creature_id, move_to).build()),
        ),
    ]
}
