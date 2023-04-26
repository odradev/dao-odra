use std::fmt::{Debug, Formatter};
use dao_contracts::{variable_repository::VariableRepositoryDeployer, VariableRepositoryRef};

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub variable_repository: VariableRepositoryRef,
}

impl DaoWorld {

}

impl Default for DaoWorld {
    fn default() -> Self {
        Self {
            variable_repository: VariableRepositoryDeployer::default(),
        }
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}