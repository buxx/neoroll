use neoroll_world::{
    entity::creature::{CreatureChange, CreatureId},
    gameplay::{
        behavior::Behavior,
        material::{Material, Resource},
    },
    space::{
        world::{MaterialChange, WorldChange},
        AbsoluteWorldPoint,
    },
};

use crate::{
    run::TICK_BASE_PERIOD,
    state::{State, StateChange},
};

use super::{ActionChange, ActionId, BodyTick, NextTick, UpdateAction};

const TICK_FREQUENCY: u64 = TICK_BASE_PERIOD;

#[derive(Debug, PartialEq)]
pub struct DropOff {
    creature_id: CreatureId,
    point: AbsoluteWorldPoint,
    material: Material,
    path: Option<Vec<AbsoluteWorldPoint>>,
}

impl DropOff {
    pub fn new(creature_id: CreatureId, point: AbsoluteWorldPoint, material: Material) -> Self {
        Self {
            creature_id,
            point,
            material,
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

impl BodyTick<DropOffChange> for DropOff {
    fn stamp(&self) -> Vec<WorldChange> {
        vec![WorldChange::Creature(
            self.creature_id,
            CreatureChange::SetBehavior(Behavior::DropOff),
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
                            NextTick(*state.frame_i() + TICK_FREQUENCY),
                            vec![
                                StateChange::World(WorldChange::Creature(
                                    self.creature_id,
                                    CreatureChange::SetPoint(next_point),
                                )),
                                StateChange::Action(
                                    id,
                                    ActionChange::Update(UpdateAction::DropOff(
                                        DropOffChange::SetPath(Some(new_path)),
                                    )),
                                ),
                            ],
                        )
                    } else {
                        // Place is busy, wait next tick
                        (NextTick(*state.frame_i() + TICK_FREQUENCY), vec![])
                    }
                } else {
                    // Path seems corrupted, try another one
                    (
                        NextTick(*state.frame_i() + TICK_FREQUENCY),
                        vec![StateChange::Action(
                            id,
                            ActionChange::Update(UpdateAction::DropOff(DropOffChange::SetPath(
                                None,
                            ))),
                        )],
                    )
                }
            } else {
                // Drop + remove this action
                let world = state.world();
                let creature = world.creatures().get(&self.creature_id).unwrap();
                let food_quantity =
                    creature.carrying_quantity(Some(Material::Resource(Resource::Food)));
                (
                    NextTick(*state.frame_i()),
                    vec![
                        StateChange::World(WorldChange::Material(
                            *creature.point(),
                            MaterialChange::Add(self.material, food_quantity.clone()),
                        )),
                        StateChange::World(WorldChange::Creature(
                            self.creature_id,
                            CreatureChange::RemoveFromCarrying(self.material, food_quantity),
                        )),
                        StateChange::Action(id, ActionChange::Remove),
                    ],
                )
            }

        // If path found, use it at next step
        } else if let Some(path) = self.find_path(state) {
            (
                NextTick(*state.frame_i() + TICK_FREQUENCY),
                vec![StateChange::Action(
                    id,
                    ActionChange::Update(UpdateAction::DropOff(DropOffChange::SetPath(Some(path)))),
                )],
            )

        // If path can be find, cancel this action
        } else {
            (
                NextTick(*state.frame_i()),
                vec![StateChange::Action(id, ActionChange::Remove)],
            )
        }
    }

    fn apply(&mut self, change: DropOffChange) {
        match change {
            DropOffChange::SetPath(path) => self.path = path,
        }
    }
}

#[derive(Debug)]
pub enum DropOffChange {
    SetPath(Option<Vec<AbsoluteWorldPoint>>),
}
