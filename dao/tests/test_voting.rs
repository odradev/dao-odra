mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber()
        // TODO: Make it work with SyncRunner
        // .before(|_feature, _rule, scenario, _world| {
        //     dbg!("Running scenario: {}", scenario.name.clone());
        //     async {
        //     }.boxed_local()
        // })
        .with_runner(cucumber_runner::SyncRunner::default())
        .run_and_exit("tests/features/voting/");
    futures::executor::block_on(runner);
}
