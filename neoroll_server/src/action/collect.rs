use neoroll_world::{
    entity::creature::{CreatureChange, CreatureId},
    gameplay::{behavior::Behavior, config::IntoCollect, material::Resource, progress::Progress},
    space::world::{FloorChange, GroundChange, StructureChange, WorldChange},
};

use crate::{
    action::{Action, ActionId, BodyTick, NextTick},
    run::TICK_BASE_PERIOD,
    state::{FrameI, State, StateChange},
};

use super::{ActionChange, UpdateAction};

const TICK_PERIOD: u64 = TICK_BASE_PERIOD / 2;

#[derive(Debug, PartialEq)]
pub struct CollectResource {
    creature_id: CreatureId,
    start: Option<FrameI>,
    end: Option<FrameI>,
    resource: Resource,
}

impl CollectResource {
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
                    // FIXME: collect duration according to resource
                    *state.frame_i() + TICK_PERIOD * 10,
                ))),
            ),
        ]
    }

    fn tick_collect(&self, state: &State) -> Vec<StateChange> {
        let mut changes = vec![];
        if let Some(progress) = self.progress(state) {
            changes.extend(vec![StateChange::World(WorldChange::Creature(
                self.creature_id,
                CreatureChange::SetBehavior(Behavior::Collect(progress)),
            ))]);

            if progress.full() {
                let world = state.world();
                let creature = world.creatures().get(&self.creature_id).unwrap();
                if let Some(structure) = &world.structure(creature.point()) {
                    if let Some(material) = structure.material(self.resource.into()) {
                        let (new_structure, collected_quantity) =
                            structure.reduced(self.resource.into());
                        if collected_quantity.0 > 0 {
                            changes.extend(vec![
                                StateChange::World(WorldChange::Structure(
                                    *creature.point(),
                                    StructureChange::Set(Some(new_structure.clone())),
                                )),
                                StateChange::World(WorldChange::Creature(
                                    self.creature_id,
                                    CreatureChange::AddToCarrying(material, collected_quantity),
                                )),
                            ]);
                            return changes;
                        }
                    }
                }
                if let Some(floor) = world.floor(creature.point()) {
                    if let Some(material) = floor.material(self.resource.into()) {
                        let (new_floor, collected_quantity) = floor.reduced(self.resource.into());
                        if collected_quantity.0 > 0 {
                            changes.extend(vec![
                                StateChange::World(WorldChange::Floor(
                                    *creature.point(),
                                    FloorChange::Set(new_floor.clone()),
                                )),
                                StateChange::World(WorldChange::Creature(
                                    self.creature_id,
                                    CreatureChange::AddToCarrying(material, collected_quantity),
                                )),
                            ]);
                            return changes;
                        }
                    }
                }
                if let Some(ground) = world.ground(creature.point()) {
                    if let Some(material) = ground.material(self.resource.into()) {
                        let (new_ground, collected_quantity) = ground.reduced(self.resource.into());
                        if collected_quantity.0 > 0 {
                            changes.extend(vec![
                                StateChange::World(WorldChange::Ground(
                                    *creature.point(),
                                    GroundChange::Set(new_ground.clone()),
                                )),
                                StateChange::World(WorldChange::Creature(
                                    self.creature_id,
                                    CreatureChange::AddToCarrying(material, collected_quantity),
                                )),
                            ]);
                            return changes;
                        }
                    }
                }
            }
        }

        changes
    }

    fn progress(&self, state: &State) -> Option<Progress> {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            let total = end.0 - start.0;
            let done = state.frame_i().0 - start.0;
            return Some(Progress::from(done as f32 / total as f32));
        }

        None
    }

    fn is_end(&self, state: &State) -> bool {
        if let Some(progress) = self.progress(state) {
            return progress.full();
        }
        false
    }
}

impl BodyTick<CollectChange> for CollectResource {
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

        // Its important to tick_collect before end to execute end progression changes
        changes.extend(self.tick_collect(state));

        if self.is_end(state) {
            changes.push(StateChange::Action(id, ActionChange::Remove));
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
    resource: Resource,
}

impl CollectBuilder {
    pub fn new(creature_id: CreatureId, resource: Resource) -> Self {
        Self {
            creature_id,
            resource,
        }
    }

    pub fn build(&self) -> Action {
        Action::Collect(CollectResource {
            creature_id: self.creature_id,
            start: Default::default(),
            end: Default::default(),
            resource: self.resource,
        })
    }
}
