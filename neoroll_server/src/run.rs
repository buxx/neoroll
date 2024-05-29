use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use neoroll_world::{
    entity::creature::{Creature, CreatureChange, CreatureId},
    space::{world::WorldChange, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI},
};
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::{
    action::{move_::MoveCreatureBuilder, Action, ActionChange, ActionId},
    state::{State, StateChange},
};

pub struct Runner {
    workers_count: usize,
    state: State,
}

impl Runner {
    pub fn new(state: State) -> Self {
        Runner {
            workers_count: num_cpus::get(),
            state,
        }
    }

    pub fn run(&mut self) {
        // HACK
        let creature_id = CreatureId::new();
        let creature_point = AbsoluteWorldPoint(AbsoluteWorldRowI(0), AbsoluteWorldColI(0));
        let move_to = AbsoluteWorldPoint(AbsoluteWorldRowI(0), AbsoluteWorldColI(30));
        self.state.apply(vec![
            StateChange::World(WorldChange::Creature(
                creature_id,
                CreatureChange::New(Creature::new(creature_id, creature_point)),
            )),
            StateChange::Action(
                ActionId::new(),
                ActionChange::New(MoveCreatureBuilder::new(creature_id, move_to).build()),
            ),
        ]);

        loop {
            let mut state_changes = vec![];
            state_changes.extend(self.tick_actions());

            self.state.apply(state_changes);
            self.state.increment();

            println!("tick");
            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn tick_actions(&self) -> Vec<StateChange> {
        let (tx, rx): (Sender<Vec<StateChange>>, Receiver<Vec<StateChange>>) = channel();
        let actions: Vec<(&ActionId, &Action)> = self.state.actions().collect();
        let state_ = &self.state;

        self.pool().scope(|s| {
            for (start, end) in self.slices(&actions) {
                let tx = tx.clone();
                let actions_ = actions.clone();

                s.spawn(move |_| {
                    for (action_id, action) in &actions_[start..end] {
                        let mut state_changes = vec![];
                        let (next, changes) = action.tick(**action_id, state_);
                        state_changes.push(StateChange::Action(
                            **action_id,
                            ActionChange::SetNextTick(next),
                        ));
                        // NOTE: It is important than SetNextTick is before because changes
                        // can contains action deletion
                        state_changes.extend(changes);
                        tx.send(state_changes).unwrap()
                    }
                })
            }
        });

        rx.try_iter()
            .collect::<Vec<Vec<StateChange>>>()
            .into_iter()
            .flatten()
            .collect()
    }

    fn slices(&self, actions: &[(&ActionId, &Action)]) -> Vec<(usize, usize)> {
        let mut slices = vec![];
        for i in 0..self.workers_count {
            let slice_len = actions.len() / self.workers_count;
            let start = slice_len * i;
            let end = if i != self.workers_count - 1 {
                start + slice_len
            } else {
                actions.len()
            };
            slices.push((start, end))
        }
        slices
    }

    fn pool(&self) -> ThreadPool {
        ThreadPoolBuilder::new()
            .num_threads(self.workers_count)
            .build()
            .unwrap()
    }
}

pub struct RunnerBuilder {
    actions: Vec<(ActionId, Action)>,
}

impl RunnerBuilder {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }

    pub fn actions(mut self, value: Vec<(ActionId, Action)>) -> Self {
        self.actions = value;
        self
    }

    pub fn build(self, mut state: State) -> Runner {
        for (action_id, action) in self.actions {
            state.apply(vec![StateChange::Action(
                action_id,
                ActionChange::New(action),
            )]);
        }

        Runner::new(state)
    }
}

impl Default for RunnerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
