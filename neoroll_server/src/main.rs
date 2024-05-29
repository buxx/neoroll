use std::sync::{Arc, RwLock};

use action::{hello::SayHelloActionBuilder, ActionId};
use neoroll_world::{map::Map, space::world::World};
use run::RunnerBuilder;
use state::State;

pub mod action;
pub mod run;
pub mod server;
mod state;

fn main() {
    let mut actions = vec![];
    for _ in 0..10_000 {
        actions.push((ActionId::new(), SayHelloActionBuilder::new().build()));
    }

    RunnerBuilder::new()
        .actions(actions)
        .build(State::default())
        .run();
}
