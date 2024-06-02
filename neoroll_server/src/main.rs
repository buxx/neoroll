use std::sync::{Arc, RwLock};

use crossbeam::channel::{unbounded, Receiver, Sender};
use neoroll_server::{
    action::{hello::SayHelloActionBuilder, ActionId},
    gateway::Gateways,
    run::RunnerBuilder,
    state::{State, StateChange},
    subscriptions::Subscriptions,
};

fn main() {
    let mut actions = vec![];
    for _ in 0..10_000 {
        actions.push((ActionId::new(), SayHelloActionBuilder::new().build()));
    }

    let subscriptions = Arc::new(RwLock::new(Subscriptions::new()));
    let gateways = Arc::new(RwLock::new(Gateways::new()));
    let (_, server_receiver): (Sender<StateChange>, Receiver<StateChange>) = unbounded();

    RunnerBuilder::new(gateways, subscriptions, server_receiver)
        .actions(actions)
        .build(State::default())
        .run();
}
