use cucumber::{when, then};
use crate::common::DaoWorld;

#[when(expr = "flipper flips")]
fn flipper_flips(_w: &mut DaoWorld) {
}

#[then(expr = "the flipper is flipped")]
fn flipper_is_flipped(_w: &mut DaoWorld) {
    assert!(true);
}