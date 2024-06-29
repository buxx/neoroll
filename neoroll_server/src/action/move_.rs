use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use neoroll_world::{
    entity::creature::{CreatureChange, CreatureId},
    gameplay::behavior::Behavior,
    space::{world::WorldChange, AbsoluteWorldPoint},
    utils::Direction,
};

use crate::{
    run::TICK_BASE_PERIOD,
    state::{State, StateChange},
};

use super::{Action, ActionChange, ActionId, BodyTick, NextTick, UpdateAction};

const TICK_PERIOD: u64 = TICK_BASE_PERIOD;

// TODO: Probable duplicate code with DropOff
#[derive(Debug, PartialEq)]
pub struct MoveTo {
    creature_id: CreatureId,
    point: AbsoluteWorldPoint,
    path: Option<Vec<AbsoluteWorldPoint>>,
}

impl MoveTo {
    pub fn new(creature_id: CreatureId, point: AbsoluteWorldPoint) -> Self {
        Self {
            creature_id,
            point,
            path: None,
        }
    }

    pub fn find_path(&self, state: &State) -> Option<Vec<AbsoluteWorldPoint>> {
        let world = state.world();
        let creature = world.creatures().get(&self.creature_id).unwrap();
        state
            .world()
            .find_path(creature.point(), &self.point)
            .map(|p| p.0)
    }
}

impl BodyTick<MoveToChange> for MoveTo {
    fn stamp(&self) -> Vec<WorldChange> {
        vec![WorldChange::Creature(
            self.creature_id,
            CreatureChange::SetBehavior(Behavior::MoveTo),
        )]
    }

    fn take_off(&self) -> Vec<WorldChange> {
        vec![WorldChange::Creature(
            self.creature_id,
            CreatureChange::SetBehavior(Behavior::Idle),
        )]
    }
    fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        if let Some(path) = &self.path {
            if let Some(try_point) = path.iter().next() {
                let mut meta = state.meta_mut();
                let world = state.world();

                if world.can_walk(try_point) {
                    if let Some(next_point) = meta.book(try_point) {
                        let new_path = path[1..].to_vec();
                        (
                            NextTick(*state.frame_i() + TICK_PERIOD),
                            vec![
                                StateChange::World(WorldChange::Creature(
                                    self.creature_id,
                                    CreatureChange::SetPoint(next_point),
                                )),
                                StateChange::Action(
                                    id,
                                    ActionChange::Update(UpdateAction::MoveTo(
                                        MoveToChange::SetPath(Some(new_path)),
                                    )),
                                ),
                            ],
                        )
                    } else {
                        // Place is busy, wait next tick
                        (NextTick(*state.frame_i() + TICK_PERIOD), vec![])
                    }
                } else {
                    // Path seems corrupted, try another one
                    (
                        NextTick(*state.frame_i() + TICK_PERIOD),
                        vec![StateChange::Action(
                            id,
                            ActionChange::Update(UpdateAction::MoveTo(MoveToChange::SetPath(None))),
                        )],
                    )
                }
            } else {
                // Drop + remove this action
                (
                    NextTick(*state.frame_i()),
                    vec![StateChange::Action(id, ActionChange::Remove)],
                )
            }

        // If path found, use it at next step
        } else if let Some(path) = self.find_path(state) {
            (
                NextTick(*state.frame_i() + TICK_PERIOD),
                vec![StateChange::Action(
                    id,
                    ActionChange::Update(UpdateAction::MoveTo(MoveToChange::SetPath(Some(path)))),
                )],
            )

        // If path cant be find, cancel this action
        } else {
            (
                NextTick(*state.frame_i()),
                vec![StateChange::Action(id, ActionChange::Remove)],
            )
        }
    }

    fn apply(&mut self, change: MoveToChange) {
        match change {
            MoveToChange::SetPath(path) => self.path = path,
        }
    }
}

#[derive(Debug)]
pub enum MoveToChange {
    SetPath(Option<Vec<AbsoluteWorldPoint>>),
}

///////

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
