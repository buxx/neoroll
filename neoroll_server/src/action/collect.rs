use neoroll_world::{
    entity::creature::{CreatureChange, CreatureId},
    gameplay::{behavior::Behavior, progress::Progress},
    space::world::WorldChange,
};

use crate::{
    action::{Action, ActionId, BodyTick, NextTick},
    state::{FrameI, State, StateChange},
};

use super::{ActionChange, UpdateAction};

const TICK_FREQUENCY: u64 = 1;

#[derive(Debug, PartialEq)]
pub struct Collect {
    creature_id: CreatureId,
    start: Option<FrameI>,
    end: Option<FrameI>,
}

impl Collect {
    fn progress(&self, state: &State) -> Vec<StateChange> {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            let total = end.0 - start.0;
            let done = state.frame_i().0 - start.0;
            let progress = Progress::from(done as f32 / total as f32);
            vec![StateChange::World(WorldChange::Creature(
                self.creature_id,
                CreatureChange::SetBehavior(Behavior::Collect(progress)),
            ))]
        } else {
            vec![]
        }
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

        if self.start.is_none() || self.end.is_none() {
            changes.extend(vec![
                StateChange::Action(
                    id,
                    ActionChange::Update(UpdateAction::Collect(CollectChange::SetStart(
                        *state.frame_i(),
                    ))),
                ),
                StateChange::Action(
                    id,
                    ActionChange::Update(UpdateAction::Collect(CollectChange::SetEnd(
                        *state.frame_i() + TICK_FREQUENCY * 10,
                    ))),
                ),
            ]);
        }

        if let Some(end) = self.end {
            if &end <= state.frame_i() {
                changes.push(StateChange::Action(id, ActionChange::Remove));
            } else {
                changes.extend(self.progress(state));
            }
        }

        (NextTick(*state.frame_i() + TICK_FREQUENCY), changes)
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
