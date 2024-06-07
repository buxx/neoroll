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
    direction: Direction,
}

impl MoveRandomly {
    pub fn new(creature_id: CreatureId, current_direction: Direction) -> Self {
        Self {
            creature_id,
            direction: current_direction,
        }
    }
}

impl BodyTick<MoveRandomlyChange> for MoveRandomly {
    fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let world = state.world();
        let creature = world.creatures().get(&self.creature_id).unwrap();
        let new_point = creature.point().next(&self.direction);

        let changes = vec![
            StateChange::World(WorldChange::Creature(
                self.creature_id,
                CreatureChange::SetPoint(new_point),
            )),
            StateChange::Action(id, ActionChange::Remove),
        ];

        (NextTick(*state.frame_i() + TICK_PERIOD), changes)
    }

    fn apply(&mut self, change: MoveRandomlyChange) {
        match change {}
    }
}

#[derive(Debug)]
pub enum MoveRandomlyChange {}

pub struct MoveRandomlyBuilder {
    creature_id: CreatureId,
    direction: Option<Direction>,
}

impl MoveRandomlyBuilder {
    pub fn new(creature_id: CreatureId) -> Self {
        Self {
            creature_id,
            direction: Default::default(),
        }
    }

    pub fn direction(mut self, value: Option<Direction>) -> Self {
        self.direction = value;
        self
    }

    pub fn build(&self) -> Action {
        let direction = match self.direction {
            Some(direction) => direction,
            None => *Direction::iter()
                .collect::<Vec<Direction>>()
                .choose(&mut rand::thread_rng())
                .unwrap_or(&Direction::Front),
        };

        Action::MoveRandomly(MoveRandomly::new(self.creature_id, direction))
    }
}
