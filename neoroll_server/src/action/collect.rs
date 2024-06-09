use neoroll_world::{
    entity::{
        creature::{CreatureChange, CreatureId},
        Filled,
    },
    gameplay::{behavior::Behavior, progress::Progress, CollectType},
    space::world::{StructureChange, WorldChange},
};

use crate::{
    action::{Action, ActionId, BodyTick, NextTick},
    run::TICK_BASE_PERIOD,
    state::{FrameI, State, StateChange},
};

use super::{ActionChange, UpdateAction};

const TICK_PERIOD: u64 = TICK_BASE_PERIOD / 2;

#[derive(Debug, PartialEq)]
pub struct Collect {
    creature_id: CreatureId,
    start: Option<FrameI>,
    end: Option<FrameI>,
}

impl Collect {
    fn is_start(&self) -> bool {
        self.start.is_none() || self.end.is_none()
    }

    fn start(&self, id: ActionId, state: &State) -> Vec<StateChange> {
        vec![
            StateChange::Action(
                id,
                ActionChange::Update(UpdateAction::Collect(CollectChange::SetStart(
                    *state.frame_i(),
                ))),
            ),
            StateChange::Action(
                id,
                ActionChange::Update(UpdateAction::Collect(CollectChange::SetEnd(
                    *state.frame_i() + TICK_PERIOD * 10,
                ))),
            ),
        ]
    }

    fn tick_collect(&self, state: &State) -> Vec<StateChange> {
        let mut changes = vec![];
        if let (Some(start), Some(end)) = (self.start, self.end) {
            let total = end.0 - start.0;
            let done = state.frame_i().0 - start.0;
            let progress = Progress::from(done as f32 / total as f32);

            changes.extend(vec![StateChange::World(WorldChange::Creature(
                self.creature_id,
                CreatureChange::SetBehavior(Behavior::Collect(progress)),
            ))]);

            if progress.full() {
                let world = state.world();
                let creature = world.creatures().get(&self.creature_id).unwrap();
                if let Some(structure) = &world.structure(creature.point()) {
                    if let Some(filled) = structure.filled() {
                        if let (Some(collect_quantity), Some(maximum_quantity)) = (
                            structure.collect_quantity(CollectType::Food),
                            structure.maximum_quantity(CollectType::Food),
                        ) {
                            let current_quantity: f32 = maximum_quantity.0 as f32 * filled.0 as f32;
                            let new_quantity_ =
                                (current_quantity as u64 - collect_quantity.0).max(0);
                            let new_filled_ =
                                ((new_quantity_ as f32 / maximum_quantity.0 as f32) * 255.) as u8;

                            let new_filled = Filled::new(new_filled_);
                            let new_structure = structure.with_filled(new_filled);
                            changes.extend(vec![StateChange::World(WorldChange::Structure(
                                *creature.point(),
                                StructureChange::Set(Some(new_structure)),
                            ))]);
                        }
                    }
                }
            }
        }

        changes
    }

    fn is_end(&self, state: &State) -> bool {
        if let Some(end) = self.end {
            return &end <= state.frame_i();
        }
        false
    }
}

impl BodyTick<CollectChange> for Collect {
    fn stamp(&self) -> Vec<WorldChange> {
        vec![WorldChange::Creature(
            self.creature_id,
            CreatureChange::SetBehavior(Behavior::Collect(Progress::from(0.))),
        )]
    }

    fn take_off(&self) -> Vec<WorldChange> {
        vec![WorldChange::Creature(
            self.creature_id,
            CreatureChange::SetBehavior(Behavior::Idle),
        )]
    }

    fn tick(&self, id: ActionId, state: &State) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];

        if self.is_start() {
            changes.extend(self.start(id, state));
        }

        if self.is_end(state) {
            changes.push(StateChange::Action(id, ActionChange::Remove));
        } else {
            changes.extend(self.tick_collect(state));
        }

        (NextTick(*state.frame_i() + TICK_PERIOD), changes)
    }

    fn apply(&mut self, change: CollectChange) {
        match change {
            CollectChange::SetStart(start) => self.start = Some(start),
            CollectChange::SetEnd(end) => self.end = Some(end),
        }
    }
}

#[derive(Debug)]
pub enum CollectChange {
    SetStart(FrameI),
    SetEnd(FrameI),
}

pub struct CollectBuilder {
    creature_id: CreatureId,
}

impl CollectBuilder {
    pub fn new(creature_id: CreatureId) -> Self {
        Self { creature_id }
    }

    pub fn build(&self) -> Action {
        Action::Collect(Collect {
            creature_id: self.creature_id,
            start: Default::default(),
            end: Default::default(),
        })
    }
}
