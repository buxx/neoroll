use action::{hello::SayHelloActionBuilder, ActionId};
use run::RunnerBuilder;

pub mod action;
pub mod run;
mod state;

fn main() {
    let mut actions = vec![];
    for _ in 0..10_000 {
        actions.push((ActionId::new(), SayHelloActionBuilder::new().build()));
    }

    RunnerBuilder::new().actions(actions).build().run();
}
