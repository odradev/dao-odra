mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;
use futures::FutureExt;

fn main() {
    let runner = DaoWorld::cucumber()
        // .after(|a, b,c, d, e| {
        //   Box::new(())
        // })
        .after(|_, _, _, _, _| {
            async {
                // odra::test_env::cleanup();
                // println!("jazda");
            }
            .boxed_local()
        })
        .run_and_exit("tests/features/ownership/");
    futures::executor::block_on(runner);
}
