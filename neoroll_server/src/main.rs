use action::{hello::SayHelloActionBuilder, ActionId};
use run::RunnerBuilder;
use state::State;

pub mod action;
pub mod gateway;
pub mod run;
pub mod server;
pub mod state;

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
