mod common;
mod steps;

use common::DaoWorld;
use cucumber::writer::Libtest;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber()
        .with_writer(Libtest::or_basic())
        .with_runner(cucumber_runner::SyncRunner::default())
        .run_and_exit("tests/features/bid_escrow/internal_worker.feature");
    futures::executor::block_on(runner);
}
