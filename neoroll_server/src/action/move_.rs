use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use neoroll_world::{
    entity::creature::{CreatureChange, CreatureId},
    space::{world::WorldChange, AbsoluteWorldPoint},
    utils::Direction,
};

use crate::{
    run::TICK_BASE_PERIOD,
    state::{State, StateChange},
};

use super::{Action, ActionChange, ActionId, BodyTick, NextTick};

#[derive(Debug, PartialEq)]
pub struct MoveTo {
    creature_id: CreatureId,
    target_point: AbsoluteWorldPoint,
}

impl MoveTo {
    pub fn new(creature_id: CreatureId, target_point: AbsoluteWorldPoint) -> Self {
        Self {
            creature_id,
            target_point,
        }
    }
}

impl BodyTick<MoveToChange> for MoveTo {
    fn tick(&self, _id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];
        let mut meta = state.meta_mut();
        let world = state.world();

        // TODO: find smooth move solution for gui
        let creature = world.creatures().get(&self.creature_id).unwrap();
        let try_point = creature.point().next(&Direction::Right);

        if let Some(next_point) = meta.book(&try_point) {
            changes.push(StateChange::World(WorldChange::Creature(
                self.creature_id,
                CreatureChange::SetPoint(next_point),
            )));
        }

        (NextTick(*state.frame_i() + 1), changes)
    }

    fn apply(&mut self, change: MoveToChange) {
        match change {}
    }
}

#[derive(Debug)]
pub enum MoveToChange {}

pub struct MoveToBuilder {
    creature_id: CreatureId,
    target_point: AbsoluteWorldPoint,
}

impl MoveToBuilder {
    pub fn new(creature_id: CreatureId, target_point: AbsoluteWorldPoint) -> Self {
        Self {
            creature_id,
            target_point,
        }
    }

    pub fn build(&self) -> Action {
        Action::MoveTo(MoveTo::new(self.creature_id, self.target_point))
    }
}

///////
const TICK_PERIOD: u64 = TICK_BASE_PERIOD;

#[derive(Debug, PartialEq)]
pub struct MoveRandomly {
    creature_id: CreatureId,
}

impl MoveRandomly {
    pub fn new(creature_id: CreatureId) -> Self {
        Self { creature_id }
    }
}

impl BodyTick<MoveRandomlyChange> for MoveRandomly {
    fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut meta = state.meta_mut();
        let world = state.world();
        let creature = world.creatures().get(&self.creature_id).unwrap();
        let mut possible_directions = Direction::iter().collect::<Vec<Direction>>();
        possible_directions.shuffle(&mut rand::thread_rng());

        while let Some(direction) = possible_directions.pop() {
            if let Some(new_point) = meta.book(&creature.point().next(&direction)) {
                if state.world().can_walk(&new_point) {
                    return (
                        NextTick(*state.frame_i() + TICK_PERIOD),
                        vec![
                            StateChange::World(WorldChange::Creature(
                                self.creature_id,
                                CreatureChange::SetPoint(new_point),
                            )),
                            StateChange::Action(id, ActionChange::Remove),
                        ],
                    );
                }
            }
        }

        (
            NextTick(*state.frame_i()),
            vec![StateChange::Action(id, ActionChange::Remove)],
        )
    }

    fn apply(&mut self, change: MoveRandomlyChange) {
        match change {}
    }
}

#[derive(Debug)]
pub enum MoveRandomlyChange {}

pub struct MoveRandomlyBuilder {
    creature_id: CreatureId,
}

impl MoveRandomlyBuilder {
    pub fn new(creature_id: CreatureId) -> Self {
        Self { creature_id }
    }

    pub fn build(&self) -> Action {
        Action::MoveRandomly(MoveRandomly::new(self.creature_id))
    }
}
