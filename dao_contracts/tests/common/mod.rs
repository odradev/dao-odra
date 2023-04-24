use std::fmt::{Debug, Formatter};
use dao_contracts::{flipper::FlipperDeployer, FlipperRef};

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub flipper: FlipperRef,
}

impl DaoWorld {

}

impl Default for DaoWorld {
    fn default() -> Self {
        Self {
            flipper: FlipperDeployer::initial_settings(),
        }
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}