use dao::core_contracts::{VariableRepositoryDeployer, VariableRepositoryRef, KycNftContractRef, VaNftContractRef, KycNftContractDeployer, VaNftContractDeployer};
use odra::test_env;
use std::fmt::{Debug, Formatter};

use self::params::Account;

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub variable_repository: VariableRepositoryRef,
    pub kyc_token: KycNftContractRef,
    pub va_token: VaNftContractRef,
}

impl DaoWorld {
    pub fn set_caller(&mut self, caller: &Account) {
        test_env::set_caller(self.get_address(caller));
    }
}

impl Default for DaoWorld {
    fn default() -> Self {
        let default_account = test_env::get_account(0);
        test_env::set_caller(default_account);
        Self {
            variable_repository: VariableRepositoryDeployer::init(
                test_env::get_account(0),
                test_env::get_account(1),
                test_env::get_account(2),
            ),
            kyc_token: KycNftContractDeployer::init("kyc_token".to_string(), "KYC".to_string(), "".to_string()),
            va_token: VaNftContractDeployer::init("va_token".to_string(), "VAT".to_string(), "".to_string()),
        }
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}


pub mod params;
pub mod helpers;
pub mod contracts;
pub mod config;