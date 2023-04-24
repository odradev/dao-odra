use cucumber::{given, when, then};
use crate::common::DaoWorld;

#[when(expr = "flipper flips")]
fn flipper_flips(w: &mut DaoWorld) {
    w.flipper.flip();
}

#[then(expr = "the flipper is flipped")]
fn flipper_is_flipped(w: &mut DaoWorld) {
    assert!(w.flipper.get());
}