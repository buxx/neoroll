use std::{
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
    time::{Duration, Instant},
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

pub const TICK_BASE_PERIOD: u64 = 50;
const SLEEP_TARGET_NS: u64 = 1_000_000_000 / TICK_BASE_PERIOD;

pub struct Runner {
    gate: Arc<RwLock<Gateways>>,
    subs: Arc<RwLock<Subscriptions>>,
    workers_count: usize,
    state: Arc<RwLock<State>>,
    server_receiver: Receiver<StateChange>,
    lag: u64,
    // stats: Arc<RwLock<Statistics>>,
}

// TODO: implement stop required (also stop statistics thread)
impl Runner {
    pub fn new(
        gateways: Arc<RwLock<Gateways>>,
        subscriptions: Arc<RwLock<Subscriptions>>,
        state: Arc<RwLock<State>>,
        server_receiver: Receiver<StateChange>,
    ) -> Self {
        Runner {
            gate: gateways,
            subs: subscriptions,
            workers_count: num_cpus::get(),
            state,
            server_receiver,
            lag: 0,
        }
    }

    fn sleep_target_ns(&self) -> u64 {
        SLEEP_TARGET_NS / self.state().game().speed()
    }

    fn state(&self) -> RwLockReadGuard<State> {
        self.state.read().unwrap()
    }

    fn state_mut(&self) -> RwLockWriteGuard<State> {
        self.state.write().unwrap()
    }

    pub fn run(&mut self) {
        // TODO: Move this code in separated code
        self.state_mut().apply(
            &self.gate,
            &self.subs,
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

        self.start_stats();

        loop {
            self.tick();
        }
    }

    fn start_stats(&self) {
        let state = Arc::clone(&self.state);

        thread::spawn(move || loop {
            let previous_frame_i = *state.read().unwrap().frame_i();
            thread::sleep(Duration::from_secs(1));

            let frame_count = state.read().unwrap().frame_i().0 - previous_frame_i.0;
            let speed = state.read().unwrap().game().speed();
            println!("{}/{} tick/s", frame_count, TICK_BASE_PERIOD * speed);
        });
    }

    fn tick(&mut self) {
        let tick_start = Instant::now();

        let mut changes = vec![];
        changes.extend(self.receive());
        changes.extend(self.tick_actions());
        self.state_mut().apply(&self.gate, &self.subs, changes);
        self.state_mut().increment();

        // FPS target
        let sleep_target = self.sleep_target_ns();
        let tick_duration = Instant::now() - tick_start;
        let need_sleep = sleep_target - (tick_duration.as_nanos() as u64).min(sleep_target);
        self.lag +=
            (tick_duration.as_nanos().max(sleep_target.into()) as u64 - sleep_target).min(0);
        let catchable_lag = self.lag.min(need_sleep);
        self.lag -= catchable_lag;
        thread::sleep(Duration::from_nanos(need_sleep - catchable_lag));
    }

    fn receive(&self) -> Vec<StateChange> {
        self.server_receiver.try_iter().collect()
    }

    fn tick_actions(&self) -> Vec<StateChange> {
        let (tx, rx): (Sender<Vec<StateChange>>, Receiver<Vec<StateChange>>) = unbounded();
        let state = self.state();
        let actions: Vec<(&ActionId, &Action)> = state.to_do().collect();

        self.pool().scope(|s| {
            let state_ = Arc::clone(&self.state);
            for (start, end) in self.slices(&actions) {
                let tx = tx.clone();
                let actions_ = actions.clone();

                let state_ = Arc::clone(&state_);
                s.spawn(move |_| {
                    let state__ = state_.read().unwrap();
                    for (action_id, action) in &actions_[start..end] {
                        let mut state_changes = vec![];
                        let (next, changes) = action.tick(**action_id, &state__);
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

        let state = Arc::new(RwLock::new(state));
        Runner::new(
            self.gateways,
            self.subscriptions,
            state,
            self.server_receiver,
        )
    }
}
