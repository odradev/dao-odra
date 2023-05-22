use dao::core_contracts::{VariableRepositoryDeployer, VariableRepositoryRef, KycNftContractRef, VaNftContractRef, KycNftContractDeployer, VaNftContractDeployer, ReputationContractRef, ReputationContractDeployer};
use odra::{test_env, types::Address};
use std::fmt::{Debug, Formatter};

use self::{params::Account, contracts::cspr::VirtualBalances};

#[derive(cucumber::World)]
pub struct DaoWorld {
    variable_repository: VariableRepositoryRef,
    kyc_token: KycNftContractRef,
    va_token: VaNftContractRef,
    reputation_token: ReputationContractRef,
    virtual_balances: VirtualBalances
}

impl DaoWorld {
    pub fn set_caller(&mut self, caller: &Account) {
        test_env::set_caller(self.get_address(caller));
    }

    pub fn variable_repository_address(&self) -> Address {
        self.variable_repository.address()
    }

    pub fn kyc_token_address(&self) -> Address {
        self.kyc_token.address()
    }

    pub fn va_token_address(&self) -> Address {
        self.va_token.address()
    }

    pub fn reputation_token_address(&self) -> Address {
        self.reputation_token.address()
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
            reputation_token: ReputationContractDeployer::init(),
            virtual_balances: Default::default()
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