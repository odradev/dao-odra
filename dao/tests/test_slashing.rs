mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber()
        .with_runner(cucumber_runner::SyncRunner::default())
        .run_and_exit("tests/features/slashing/slashing_voter_full_slash.feature");
    futures::executor::block_on(runner);
}
