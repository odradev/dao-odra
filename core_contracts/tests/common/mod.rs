use core_contracts::{VariableRepositoryDeployer, VariableRepositoryRef};
use odra::test_env;
use std::fmt::{Debug, Formatter};

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub variable_repository: VariableRepositoryRef,
}

impl DaoWorld {}

impl Default for DaoWorld {
    fn default() -> Self {
        Self {
            variable_repository: VariableRepositoryDeployer::init(
                test_env::get_account(0),
                test_env::get_account(1),
                test_env::get_account(2),
            ),
        }
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}
