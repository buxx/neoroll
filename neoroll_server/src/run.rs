use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::{
    action::{
        client::ComputeAndSendClientStates, migrant::IncomingMigrant, Action, ActionChange,
        ActionId,
    },
    gateway::Gateways,
    state::{State, StateChange},
    subscriptions::Subscriptions,
};

pub struct Runner {
    gateways: Arc<RwLock<Gateways>>,
    subscriptions: Arc<RwLock<Subscriptions>>,
    workers_count: usize,
    state: State,
    server_receiver: Receiver<StateChange>,
}

impl Runner {
    pub fn new(
        gateways: Arc<RwLock<Gateways>>,
        subscriptions: Arc<RwLock<Subscriptions>>,
        state: State,
        server_receiver: Receiver<StateChange>,
    ) -> Self {
        Runner {
            gateways,
            subscriptions,
            workers_count: num_cpus::get(),
            state,
            server_receiver,
        }
    }

    pub fn run(&mut self) {
        self.state.apply(
            &self.gateways,
            &self.subscriptions,
            vec![
                StateChange::Action(
                    ActionId::new(),
                    ActionChange::New(Action::ComputeAndSendClientStates(
                        ComputeAndSendClientStates,
                    )),
                ),
                StateChange::Action(
                    ActionId::new(),
                    ActionChange::New(Action::IncomingMigrant(IncomingMigrant)),
                ),
            ],
        );

        loop {
            let mut changes = vec![];
            changes.extend(self.receive());
            changes.extend(self.tick_actions());

            self.state
                .apply(&self.gateways, &self.subscriptions, changes);
            self.state.increment();

            println!("tick");
            thread::sleep(Duration::from_millis(250));
        }
    }

    fn receive(&self) -> Vec<StateChange> {
        self.server_receiver.try_iter().collect()
    }

    fn tick_actions(&self) -> Vec<StateChange> {
        let (tx, rx): (Sender<Vec<StateChange>>, Receiver<Vec<StateChange>>) = unbounded();
        let actions: Vec<(&ActionId, &Action)> = self.state.to_do().collect();
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
    gateways: Arc<RwLock<Gateways>>,
    subscriptions: Arc<RwLock<Subscriptions>>,
    actions: Vec<(ActionId, Action)>,
    server_receiver: Receiver<StateChange>,
}

impl RunnerBuilder {
    pub fn new(
        gateways: Arc<RwLock<Gateways>>,
        subscriptions: Arc<RwLock<Subscriptions>>,
        server_receiver: Receiver<StateChange>,
    ) -> Self {
        Self {
            gateways,
            subscriptions,
            actions: vec![],
            server_receiver,
        }
    }

    pub fn actions(mut self, value: Vec<(ActionId, Action)>) -> Self {
        self.actions = value;
        self
    }

    pub fn build(self, mut state: State) -> Runner {
        for (action_id, action) in self.actions {
            state.apply(
                &self.gateways,
                &self.subscriptions,
                vec![StateChange::Action(action_id, ActionChange::New(action))],
            );
        }

        Runner::new(
            self.gateways,
            self.subscriptions,
            state,
            self.server_receiver,
        )
    }
}
