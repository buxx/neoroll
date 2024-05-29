use neoroll_world::{
    entity::creature::{CreatureChange, CreatureId},
    space::{world::WorldChange, AbsoluteWorldPoint},
    utils::Direction,
};

use crate::state::{State, StateChange};

use super::{Action, ActionId, BodyTick, NextTick};

#[derive(Debug, PartialEq)]
pub struct MoveCreature {
    creature_id: CreatureId,
    target_point: AbsoluteWorldPoint,
}

impl MoveCreature {
    pub fn new(creature_id: CreatureId, target_point: AbsoluteWorldPoint) -> Self {
        Self {
            creature_id,
            target_point,
        }
    }
}

impl BodyTick<MoveCreatureChange> for MoveCreature {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];
        let world = state.world();

        // TODO: find smooth move solution for gui
        let creature = world.creatures().get(&self.creature_id).unwrap();
        let new_point = creature.point().next(&Direction::Right);
        changes.push(StateChange::World(WorldChange::Creature(
            self.creature_id,
            CreatureChange::SetPoint(new_point),
        )));

        (NextTick(*state.frame_i() + 1), changes)
    }

    fn apply(&mut self, change: MoveCreatureChange) {
        match change {}
    }
}

pub enum MoveCreatureChange {}

pub struct MoveCreatureBuilder {
    creature_id: CreatureId,
    target_point: AbsoluteWorldPoint,
}

impl MoveCreatureBuilder {
    pub fn new(creature_id: CreatureId, target_point: AbsoluteWorldPoint) -> Self {
        Self {
            creature_id,
            target_point,
        }
    }

    pub fn build(&self) -> Action {
        Action::MoveCreature(MoveCreature::new(self.creature_id, self.target_point))
    }
}
